# git-cm

> A git subcommand for creating conventional-friendly commit messages.

## Installation

Either compile from source or install via [crates.io](https://crates.io):

```console
$ cargo install git-cm --locked
```

For macOS, you can install `git-cm` via homebrew:

```console
$ brew install sirwindfield/tap/git-cm
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

#### License

<sup>
Licensed under either of <a href="license-apache">Apache License, Version
2.0</a> or <a href="license-mit">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
