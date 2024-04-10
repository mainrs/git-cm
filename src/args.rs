use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version)]
pub struct App {
    /// Opens the user's editor after the questioning process.
    #[arg(long, short = 'e')]
    pub edit: bool,
    /// The path to a git repository.
    #[arg(name = "REPO", default_value = ".")]
    pub repo_path: PathBuf,
}
