# zerotask-rust-lib-template

[![docs_master_badge]][docs_master_url]

> A GitHub template for Rust libraries.

## Features

- Continuous Integration through GitHub Actions
  - Each PR is tested by running the following commands to ensure that only working code is added to the repository:
    - `cargo fmt` to ensure uniform source code formatting.
    - `cargo clippy` to use more idiomic Rust code, optimize code as well as prevent hard to spot bugs.
    - `cargo check` to ensure that the library compiles properly.
    - `cargo test` to ensure that the library works as expected.
  - Each push to master triggers the following:
    - Generation of the newest documentation that gets pushed to the `gh-pages` branch.
- MSRV (**M**inimal **s**upported **R**ust **v**ersion)
  - Kept in sync with the latest available Rust version on Ubuntu.
- Opinioded `rustfmt` configuration file.
- Misc
  - `.editorconfig` file for code-unrelated files.
    - Ensures proper formatting for workflow files and other configuration files.

## Current Properties

- MSRV: 1.41.0

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
