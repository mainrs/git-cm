use crate::{
    args::App,
    cargo::parse_manifest,
    git::{
        check_staged_files_exist, commit_to_repo, generate_commit_msg, get_repository,
        DEFAULT_TYPES,
    },
    questions::{ask, SurveyResults},
};
use anyhow::{Error, Result};
use clap::Clap;
use dialoguer::{theme::ColorfulTheme, Confirm};
use itertools::Itertools;
use self_update::{cargo_crate_version, version::bump_is_greater};
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

fn create_commit(commit_msg: &str, repo: &Path) {
    let hash = commit_to_repo(commit_msg, repo).expect("Failed to create commit");
    println!("Wrote commit: {}", hash);
}

fn update_app() -> Result<(), Error> {
    // Check if the latest release published is newer than the actual one which
    // is currently being used.
    match look_for_new_release() {
        Ok(None) => Ok(()),
        Ok(Some(_)) => {
            let want_to_bump = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(
                    "A newer version for this crate was found.\nWould you like to update your application?"
                )
                .default(false)
                .interact()?;
            if want_to_bump == true {
                let status = self_update::backends::github::Update::configure()
                    .repo_owner("SirWindfield")
                    .repo_name("git-cm")
                    .bin_name("git-cm")
                    .show_download_progress(true)
                    .show_output(true)
                    .current_version(cargo_crate_version!())
                    .build()?
                    .update()?;
                println!(
                    "You've sucessfully updated your version to `{}`!",
                    status.version()
                );
                Ok(())
            } else {
                Ok(())
            }
        }
        Err(e) => Err(e),
    }
}

fn look_for_new_release() -> Result<Option<String>> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("SirWindfield")
        .repo_name("git-cm")
        .build()?
        .fetch()?;
    if bump_is_greater(cargo_crate_version!(), &releases[0].version)? {
        Ok(Some(releases[0].version.to_string()))
    } else {
        Ok(None)
    }
}

fn run(app: App) {
    // Check if there's a new release of the crate. In that case, ask the user
    // if he/she wants to update it.
    match update_app() {
        Ok(()) => (),
        Err(e) => eprintln!("An error ocurred during the app update process: {}", e),
    };

    // No point to continue if repo doesn't exist or there are no staged files
    if check_staged_files_exist(app.repo_path.as_path()) {
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
            Some(msg) => create_commit(&msg, app.repo_path.as_path()),
            None => eprintln!("Empty commit message specified!"),
        }
    } else {
        eprintln!("Nothing to commit!");
    }
}

fn main() {
    let app: App = App::parse();
    // Early return if the path doesn't exist.
    if !app.repo_path.exists() || get_repository(app.repo_path.as_path()).is_err() {
        eprintln!("Invalid path to repository: {}", app.repo_path.display());
    } else {
        run(app);
    }
}
