use crate::questions::SurveyResults;
use crate::{
    args::App,
    cargo::parse_manifest,
    git::{commit_to_repo, generate_commit_msg, DEFAULT_TYPES},
    questions::ask,
};
use clap::Clap;
use itertools::Itertools;
use std::{collections::HashMap, path::Path};

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

fn create_commit(commit_msg: &str, repository_path: impl AsRef<Path>) {
    let hash =
        commit_to_repo(commit_msg, repository_path.as_ref()).expect("Failed to create commit");
    println!("Wrote commit: {}", hash);
}

fn run(app: App) {
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
        Some(msg) => create_commit(&msg, app.repo_path),
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
