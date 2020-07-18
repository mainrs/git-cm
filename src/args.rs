use clap::Clap;
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap(author, version)]
pub struct App {
    #[clap(long, short = "e")]
    pub edit: bool,
    #[clap(name = "REPO", default_value = ".", parse(from_os_str))]
    pub repo_path: PathBuf,
}
