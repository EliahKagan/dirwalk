fn main() -> anyhow::Result<()> {
    let repo = gix::discover(".")?;
    let options = repo.dirwalk_options()?;
    println!("{options:?}");
    Ok(())
}
