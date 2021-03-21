use clap::Clap;
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap(author, version)]
pub struct App {
    /// Opens the user's editor to type the commit message instead of showing a
    /// dialog.
    #[clap(long, short = 'e')]
    pub edit: bool,
    /// The path to a git repository.
    #[clap(name = "REPO", default_value = ".", parse(from_os_str))]
    pub repo_path: PathBuf,
}
