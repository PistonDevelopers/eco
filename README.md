# eco
[![Build Status](https://travis-ci.org/PistonDevelopers/eco.svg?branch=master)](https://travis-ci.org/PistonDevelopers/eco)

A tool for reasoning about breaking changes in Rust ecosystems

Eco processes custom JSON formats about Rust ecosystems.

Currently supported:

- Extract info: A list of urls to extract dependency information from Cargo.toml.
- Dependency info: Version info about packages and their dependencies.
- Update info: Actions to improve the integration of the ecosystem.

### Help out with updating!

Breaking changes happen all the time, and keeping libraries updated is important.
Here we keep a list of various ecosystems important for Rust gamedev.

- Piston: `cargo run --example piston > todo.txt`

Eco generates a list of updates, which you follow as instructions and then make PR to the respective repos.

### Motivation

Rust ecosystems often consist of many smaller crates following semver versioning.
When the first non-zero number changes, it means a breaking change.
Depending on the shape and size of your ecosystem, different breaking changes have different consequences.

Keeping an ecosystem integrated is a huge task, and Piston is part of a large ecosystem,
even extending beyond the PistonDevelopers organization.
What matters most is that existing code continues working, and that updates happen soon after changes are made.
Ideally, to avoid dependency conflicts and large binaries, the ecosystem should use the same versions of libraries.
This is a hard mental task to do manually, and almost impossible to do without making mistakes.

Eco is designed to complement [other tools](https://github.com/PistonDevelopers/eco/issues/20) for Rust ecosystems.
It can extract dependency information directly from Cargo.toml,
then run an analysis on the current state and output recommended actions.
These actions can then be used by to assist maintainers in their work, or perhaps automate some tasks in the future.

Eco uses [Piston-Meta](https://github.com/pistondevelopers/meta) for parsing text.
Meta parsing is a techinque where data from arbitrary text can be queried using a "meta syntax",
supporting a large number of formats for specific purposes using a single library.
This allows quick fixes to custom formats, validation of structure, and gives good error messages.
One sub-goal of this project is to test and improve Piston-Meta for use in infrastructure,
where various parts are interfacing each other through text.

Because Eco might be used for automation in the future, the algorithms are based on analysis and models.
When something goes wrong, it should be known what error might have caused it.
This is necessary to use it with other tools, so the overall behavior can be reasoned about.

### OpenSSL issues on macOS

On macOS, if you use Homebrew or MacPorts to install OpenSSL, you may need to add the following lines
to your `.bash_profile` to resolve compile-time errors:

```
export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
export DEP_OPENSSL_INCLUDE=`brew --prefix openssl`/include
```

## Making pull requests

Prior to making a pull request on eco, make sure that you have `cargo fmt` against the
codebase. Instructions to install `rustfmt` can be found [here](https://github.com/rust-lang-nursery/rustfmt).
Ensure that you are installing the nightly version, since there is currently an effort
to port `rustfmt` away from the `syntex` crate, and `rustfmt-nightly` is the latest version.

Unfortunately, this also means that you will have to switch over to nightly prior to
running `cargo fmt` against the codebase.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
