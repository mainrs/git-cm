use once_cell::sync::Lazy;

use std::collections::HashMap;

pub(crate) static DEFAULT_TYPES: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
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
