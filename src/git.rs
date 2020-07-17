use crate::questions::SurveyResults;
use anyhow::{anyhow, Result};
use git2::{Commit, ObjectType, Oid, Repository};
use once_cell::sync::Lazy;
use std::collections::HashMap;

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

/// Generates the commit message by selectively appending all parts that the user entered.
pub fn generate_commit_msg(survey: SurveyResults) -> String {
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
/// # Returns
///
/// The hash of the added commit.
///
/// # Note
///
/// The method uses the default username and email address found for the repository. Defaults to the globally configured when needed.
pub fn commit(msg: String) -> Result<Oid> {
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
