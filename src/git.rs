use crate::questions::SurveyResults;
use anyhow::{anyhow, Result};
use git2::{Commit, ObjectType, Oid, Repository};
use once_cell::sync::Lazy;
use std::{collections::HashMap, path::PathBuf};

/// All default conventional commit types alongside their description.
pub static DEFAULT_TYPES: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert("build", "Changes that affect the build system or external dependencies (example scopes: cargo, bazel, make)");
    m.insert("chore", "Other changes that don't modify src or test files");
    m.insert("ci", "Changes to our CI configuration files and scripts (example scopes: Travis, Circle, GitHub Actions)");
    m.insert("docs", "Documentation only changes");
    m.insert("feat", "A new feature");
    m.insert("fix", "A bug fix");
    m.insert("perf", "A code change that improves performance");
    m.insert(
        "refactor",
        "A code change that neither fixes a bug nor adds a feature",
    );
    m.insert("revert", "Reverts a previous commit");
    m.insert(
        "style",
        "Changes that do not affect the meaning of the code (white-space, formatting, etc)",
    );
    m.insert("test", "Adding missing tests or correcting existing tests");

    m
});

fn format_footer(commit_type: &str, issues_list: &[String]) -> String {
    let footer_key = match commit_type {
        "fix" => "Fixes",
        "feat" => "Closes",
        _ => "Referenced-issues",
    };
    let mut footer_value = String::new();
    issues_list.iter().for_each(|s| {
        footer_value.push_str(s);
        footer_value.push(' ')
    });

    let footer_separator = match footer_value.chars().next() {
        Some('#') => " ",
        _ => ": ",
    };

    format!("{}{}{}", footer_key, footer_separator, footer_value)
}

/// Generates the commit message by selectively appending all parts that the
/// user entered.
pub fn generate_commit_msg(survey: SurveyResults) -> String {
    let commit_type_and_scope = match survey.scope {
        Some(scope) => format!("{}({})", survey.commit_type, scope),
        None => survey.commit_type.to_owned(),
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
        Some(issues_list) => format!(
            "{}\n\n{}",
            with_breaking_change,
            format_footer(&survey.commit_type, &issues_list)
        ),
        None => with_breaking_change,
    }
}

///Finds the last commit inside a git repository.
fn find_last_commit(repo: &Repository) -> Result<Commit> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| anyhow!("could not find commit"))
}

/// Commits the added changes to the repository.
///
/// # Arguments
///
/// - `msg`: The commit message to use.
///
/// - `repository`: Path to the git repository
///
/// # Returns
///
/// The hash of the added commit.
///
/// # Note
///
/// The method uses the default username and email address found for the
/// repository. Defaults to the globally configured when needed.
pub fn commit(msg: String, repository: PathBuf) -> Result<Oid> {
    let repo = Repository::open(repository.as_os_str()).expect("Failed to open git repository");

    let mut index = repo.index()?;
    let oid = index.write_tree()?;
    let signature = repo.signature()?;

    let tree = repo.find_tree(oid)?;
    let parent_commit = find_last_commit(&repo);

    let oid = match parent_commit {
        Ok(parent_commit) => repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &msg,
            &tree,
            &[&parent_commit],
        )?,
        Err(_) => repo.commit(Some("HEAD"), &signature, &signature, &msg, &tree, &[])?,
    };

    Ok(oid)
}
