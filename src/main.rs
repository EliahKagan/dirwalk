use gix::dir::walk::{CollapsedEntriesEmissionMode, EmissionMode};

fn main() -> anyhow::Result<()> {
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
    let index = repo.index()?;
    for item in repo.dirwalk_iter(index, Vec::<&str>::new(), Default::default(), options)? {
        println!("{:?}", item?.entry);
    }
    Ok(())
}
