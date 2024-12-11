use std::env::args_os;

use anyhow::Result;
use bstr::{BString, ByteSlice};
use gix::dir::walk::{CollapsedEntriesEmissionMode, EmissionMode};
use gix::dirwalk::Iter;

fn make_dirwalk_iterator<S, P>(patterns: P) -> Result<Iter>
where
    S: Into<BString>,
    P: IntoIterator<Item = S>,
{
    let repo = gix::discover(".")?;

    let options: gix::dirwalk::Options = repo
        .dirwalk_options()?
        .recurse_repositories(false)
        .emit_pruned(true)
        .emit_ignored(Some(EmissionMode::Matching))
        .for_deletion(None)
        .emit_tracked(true)
        .emit_untracked(EmissionMode::Matching)
        .emit_empty_directories(true)
        .classify_untracked_bare_repositories(true)
        .emit_collapsed(Some(CollapsedEntriesEmissionMode::All))
        .symlinks_to_directories_are_ignored_like_directories(false)
        .empty_patterns_match_prefix(false);

    Ok(repo.dirwalk_iter(repo.index()?, patterns, Default::default(), options)?)
}

fn build_table(patterns: impl Iterator<Item = BString>) -> Result<Vec<(String, String)>> {
    let mut table = Vec::new();

    for item in make_dirwalk_iterator(patterns)? {
        let entry = item?.entry;
        let status = format!("{:?}", entry.status);
        let path = format!("{}", entry.rela_path);
        table.push((status, path));
    }

    Ok(table)
}

fn print_table(table: &Vec<(String, String)>) {
    let status_width = table
        .iter()
        .map(|(status, _)| status.chars().count())
        .max()
        .unwrap_or(1);

    for (status, path) in table {
        println!("{status:>width$}    {path}", width = status_width);
    }
}

fn main() -> Result<()> {
    let patterns = args_os()
        .skip(1)
        .map(|p| p.as_encoded_bytes().as_bstr().to_owned());
    print_table(&build_table(patterns)?);
    Ok(())
}
