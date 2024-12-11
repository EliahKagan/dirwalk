use anyhow::Result;
use gix::dir::walk::{CollapsedEntriesEmissionMode, EmissionMode};
use gix::dirwalk::Iter;

fn make_dirwalk_iterator() -> Result<Iter> {
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

    Ok(repo.dirwalk_iter(
        repo.index()?,
        Vec::<&str>::new(),
        Default::default(),
        options,
    )?)
}

fn main() -> Result<()> {
    for item in make_dirwalk_iterator()? {
        println!("{:?}", item?.entry);
    }
    Ok(())
}
