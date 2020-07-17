# git-cm

[![docs_master_badge]][docs_master_url]

> A git subcommand for creating conventional-friendly commit messages.

## Installation

Either compile from source or install via [crates.io](https://crates.io):

```text
cargo install git-cm
```

## Usage

Instead of using `git commit` to commit changes, simply run `git cm`. This will start the questioning process and commit the message once you're done.

You also have to specifiy which types of commits your project supports. Just add the following to your `Cargo.toml`:

```toml
[package.metadata.commits]
defaults = true

# This is optional
[[package.metadata.commits.type]]
name = "xyz"
desc = "A custom command"
```

## Example run

![Example run GIF](.github/git-cm.gif)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[docs_master_badge]: https://img.shields.io/badge/docs.rs-master-green
[docs_master_url]: https://<username>.github.io/<reponame>
