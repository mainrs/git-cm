use crate::commit::DEFAULT_TYPES;
use anyhow::{anyhow, Result};
use cargo_toml::Manifest;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use git2::{Commit, ObjectType, Oid, Repository};
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

mod commit;

#[derive(Debug, Deserialize)]
struct Metadata {
    pub commits: CzMetadata,
}

#[derive(Debug, Deserialize)]
struct CzMetadata {
    pub defaults: bool,
    pub r#type: Option<Vec<CzTypeMetadata>>,
}

#[derive(Debug, Deserialize)]
struct CzTypeMetadata {
    name: String,
    desc: String,
}

#[derive(Debug, Default)]
struct SurveyResults {
    commit_type: String,
    scope: Option<String>,
    short_msg: String,
    long_msg: Option<String>,
    breaking_changes_desc: Option<String>,
    affected_open_issues: Option<String>,
}

impl SurveyResults {
    pub fn new() -> Self {
        Self::default()
    }
}

fn parse_manifest() -> Result<Manifest<Metadata>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let manifest: Manifest<Metadata> = Manifest::from_path_with_metadata(path)?;

    Ok(manifest)
}

fn run_dialog(types: HashMap<&str, &str>) -> SurveyResults {
    let mut results = SurveyResults::new();

    // Select the scope of the commit.
    let type_options = types
        .iter()
        .map(|(name, desc)| (name, desc))
        .collect::<Vec<_>>();
    let items = type_options
        .iter()
        .map(|(name, desc)| format!("{}: {}", name, desc))
        .collect::<Vec<_>>();

    let selected_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the type of change that you're committing:")
        .default(0)
        .items(&items)
        .interact()
        .unwrap();
    let selected_commit_type = &type_options[selected_index];
    results.commit_type = selected_commit_type.0.to_string();

    let scope = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Denote the scope of this change (compiler, runtime, stdlib, etc.):")
        .allow_empty(true)
        .interact()
        .ok();
    results.scope = scope;

    let short_msg: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Write a short, imperative tense description of the change:")
        .allow_empty(true)
        .interact()
        .unwrap();
    results.short_msg = short_msg;

    let long_msg: Option<String> = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Provide a longer description of the change:")
        .allow_empty(true)
        .interact()
        .ok()
        .filter(|v: &String| !v.is_empty());
    results.long_msg = long_msg;

    let is_breaking_change = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Are there any breaking changes?")
        .default(false)
        .interact()
        .unwrap();

    if is_breaking_change {
        let breaking_changes_desc = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Describe the breaking changes:")
            .interact()
            .ok();
        results.breaking_changes_desc = breaking_changes_desc;
    }

    let are_issues_affected = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Does this change affect any open issues?")
        .default(false)
        .interact()
        .unwrap();

    if are_issues_affected {
        let affected_open_issues = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Add issue references (e.g. \"fix #123\", \"re #123\"):")
            .interact()
            .ok();
        results.affected_open_issues = affected_open_issues;
    }

    results
}

fn generate_commit_msg(survey: SurveyResults) -> String {
    let commit_type_and_scope = match survey.scope {
        Some(scope) => format!("{}({})", survey.commit_type, scope),
        None => survey.commit_type,
    };
    let pre_colon = match survey.breaking_changes_desc {
        Some(_) => format!("{}!", commit_type_and_scope),
        None => commit_type_and_scope,
    };
    let with_short_msg = format!("{}: {}", pre_colon, survey.short_msg);
    let with_long_msg = match survey.long_msg {
        Some(long_msg) => format!("{}\n\n{}", with_short_msg, long_msg),
        None => with_short_msg,
    };
    let with_breaking_change = match survey.breaking_changes_desc {
        Some(desc) => format!("{}\n\nBREAKING CHANGE: {}", with_long_msg, desc),
        None => with_long_msg,
    };
    match survey.affected_open_issues {
        Some(issue_list) => format!("{}\n\n{}", with_breaking_change, issue_list),
        None => with_breaking_change,
    }
}

fn find_last_commit(repo: &Repository) -> Result<Commit> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| anyhow!("could not find commit"))
}

fn commit(msg: String) -> Result<Oid> {
    let repo_root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let repo = Repository::open(repo_root.as_str()).expect("Failed to open git repository");

    let mut index = repo.index()?;
    let oid = index.write_tree()?;
    let signature = repo.signature()?;
    let parent_commit = find_last_commit(&repo)?;
    let tree = repo.find_tree(oid)?;

    let oid = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &msg,
        &tree,
        &[&parent_commit],
    )?;

    Ok(oid)
}

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

            let survey = run_dialog(types);
            let commit_msg = generate_commit_msg(survey);
            let hash = commit(commit_msg).expect("Failed to create commit");
            println!("Wrote commit: {}", hash);
        } else {
            eprintln!("Please specify allowed scopes inside of your Cargo.toml file under the `package.metadata.cz` key!");
        }
    }
}
