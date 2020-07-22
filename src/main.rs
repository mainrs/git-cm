use crate::{
    args::App,
    cargo::parse_manifest,
    git::{commit_to_repo, generate_commit_msg, DEFAULT_TYPES},
    questions::{ask, SurveyResults},
};
use clap::Clap;
use git2::{Repository, RepositoryOpenFlags, Status};
use itertools::Itertools;
use std::{collections::HashMap, ffi::OsStr};

mod args;
mod cargo;
mod git;
mod questions;
mod util;

fn run_dialog() -> Option<SurveyResults> {
    let manifest = parse_manifest().unwrap();
    if let Some(package) = manifest.package {
        if let Some(metadata) = package.metadata {
            // Use default scopes and/or custom ones.
            let mut types: HashMap<&str, &str> = HashMap::with_capacity(10);
            if metadata.commits.defaults {
                types.extend(&*DEFAULT_TYPES);
            }

            // Insert custom types.
            if let Some(custom_types) = &metadata.commits.r#type {
                for r#type in custom_types.iter() {
                    types.insert(&r#type.name, &r#type.desc);
                }
            }

            return Some(ask(types));
        } else {
            eprintln!("Please specify allowed scopes inside of your Cargo.toml file under the `package.metadata.cz` key!");
        }
    }

    None
}

fn create_commit(commit_msg: &str, repo: &Repository) {
    let hash = commit_to_repo(commit_msg, repo).expect("Failed to create commit");
    println!("Wrote commit: {}", hash);
}

fn run(app: App) {
    // No point doing anything if we're not in a Git repo
    //let repo_path: Path = app.repo_path;
    let repo = Repository::open_ext(
        app.repo_path.as_path().as_os_str(),
        RepositoryOpenFlags::empty(),
        vec![OsStr::new("")],
    )
    .expect("Failed to open git repository");

    // No point doing anything if there are no staged files
    match repo.statuses(Option::None) {
        Ok(s) => {
            if s.iter().fold(true, |acc, se| {
                acc & ((se.status() == Status::CURRENT) | (se.status() == Status::IGNORED))
            }) {
                panic!("Error: nothing to commit")
            }
        }
        Err(e) => panic!("Error: {}", e),
    };

    // We can short-hand the editor mode for now as there aren't type-agnostic
    let commit_msg = if app.edit {
        let template = include_str!("../assets/editor_template.txt");
        edit::edit(template)
            .ok()
            .map(|v| {
                let lines = util::LinesWithEndings::from(&v);
                lines.filter(|v| !v.starts_with('#')).join("")
            })
            .filter(|v| !v.trim().is_empty())
    } else {
        let survey = run_dialog();
        survey.map(generate_commit_msg)
    };

    match commit_msg {
        Some(msg) => create_commit(&msg, &repo),
        None => eprintln!("Empty commit message specified!"),
    }
}

fn main() {
    let app: App = App::parse();
    // Early return if the path doesn't exist.
    if !app.repo_path.exists() {
        eprintln!("Invalid path to repository: {}", app.repo_path.display());
    } else {
        run(app);
    }
}
