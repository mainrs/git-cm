use crate::config::parse_manifest;
use crate::git::{commit, generate_commit_msg, DEFAULT_TYPES};
use crate::questions::ask;
use std::collections::HashMap;

mod config;
mod git;
mod questions;

fn main() {
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

            let survey = ask(types);
            let commit_msg = generate_commit_msg(survey);
            let hash = commit(commit_msg).expect("Failed to create commit");
            println!("Wrote commit: {}", hash);
        } else {
            eprintln!("Please specify allowed scopes inside of your Cargo.toml file under the `package.metadata.cz` key!");
        }
    }
}
