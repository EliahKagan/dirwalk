fn main() -> anyhow::Result<()> {
    let repo = gix::discover(".")?;
    let options: gix::dirwalk::Options = repo
        .dirwalk_options()?
        .recurse_repositories(false)
        .emit_pruned(false)
        .emit_ignored(None)
        .for_deletion(None)
        .emit_untracked(gix::dir::walk::EmissionMode::Matching)
        .emit_empty_directories(false)
        .classify_untracked_bare_repositories(true)
        .emit_collapsed(None)
        .symlinks_to_directories_are_ignored_like_directories(false)
        .empty_patterns_match_prefix(true);
    let index = repo.index()?;
    for item in repo.dirwalk_iter(index, vec![":(top)"], Default::default(), options)? {
        println!("{:?}", item?.entry);
    }
    Ok(())
}
