#![deny(missing_docs)]

//! A tool for reasoning about breaking changes in Rust ecosystems
//!
//! Eco helps Rust programmers to keep an ecosystem updated.
//! An ecosystem is a collection of libraries that are relevant to a project.
//!
//! This library is organized into modules, where each module has its own
//! custom text format. By using text, it is easy to customize the
//! collecting and generating of data.
//!
//! ### Ecosystem
//!
//! The definition of an ecosystem used by Eco is:
//!
//! ```test
//! A list of libraries, each with a set of dependencies,
//! forming an directed acyclic graph for dependencies and directed cyclic
//! graph for dev-dependencies, with no holes.
//! ```
//!
//! ### Example
//!
//! Extract info is a bird view of an ecosystem of libraries.
//! This is used to build a dependency graph, which then is used to generate
//! recommended update actions to keep the ecosystem healthy.
//!
//! ```ignore
//! extern crate eco;
//!
//! fn main() {
//!     use std::io::Read;
//!     use std::fs::File;
//!
//!     // Load extract info from file.
//!     let mut extract_info_file = File::open("assets/extract/piston.txt").unwrap();
//!     let mut extract_info = String::new();
//!     extract_info_file.read_to_string(&mut extract_info).unwrap();
//!
//!     let dependency_info = eco::extract::extract_dependency_info_from(&extract_info).unwrap();
//!     let update_info = eco::update::generate_update_info_from(&dependency_info).unwrap();
//!     println!("{}", update_info);
//! }
//! ```
//!
//! ### Unsoundness of holes
//!
//! It is important to not leave any hole in the ecosystem.
//! A hole is when a dependency is not listed that uses a listed library.
//!
//! Example:
//!
//! ```text
//! A (listed) -> B (not listed) -> C (listed)
//! ```
//!
//! The dependencies of library B will not be analyzed because they are not
//! listed. This will lead to a potential error since breaking changes in
//! library C does not cause a breaking change in A.
//!
//! Notice it is OK to not list libraries that are lower level dependencies.
//!
//! As long as there are no holes, the update algorithm is sound.

extern crate range;
extern crate piston_meta;
extern crate hyper;
extern crate semver;

pub mod extract;
pub mod update;
pub mod dependencies;
pub mod todo;

#[cfg(test)]
mod tests {
    use piston_meta::*;

    #[test]
    fn extract_is_json() {
        let _ = load_syntax_data("assets/json/syntax.txt",
            "assets/extract/test.txt");
    }

    #[test]
    fn extract() {
        let _data = load_syntax_data("assets/extract/syntax.txt",
            "assets/extract/test.txt");
        let _data = load_syntax_data("assets/extract/syntax.txt",
            "assets/extract/test2.txt");
            let _data = load_syntax_data("assets/extract/syntax.txt",
                "assets/extract/test3.txt");
        // json::print(&_data);
    }

    #[test]
    fn dependencies_is_json() {
        let _ = load_syntax_data("assets/json/syntax.txt",
            "assets/dependencies/test.txt");
        let _ = load_syntax_data("assets/json/syntax.txt",
            "assets/dependencies/test2.txt");
        let _ = load_syntax_data("assets/json/syntax.txt",
            "assets/dependencies/test3.txt");
    }

    #[test]
    fn dependencies() {
        let _data = load_syntax_data("assets/dependencies/syntax.txt",
            "assets/dependencies/test.txt");
        let _data = load_syntax_data("assets/dependencies/syntax.txt",
            "assets/dependencies/test2.txt");
        let _data = load_syntax_data("assets/dependencies/syntax.txt",
            "assets/dependencies/test3.txt");
        let _data = load_syntax_data("assets/dependencies/syntax.txt",
            "assets/dependencies/test4.txt");
        // json::print(&_data);
    }

    #[test]
    fn cargo_toml() {
        let _data = load_syntax_data("assets/cargo-toml/syntax.txt",
            "assets/cargo-toml/test.txt");
        let _data = load_syntax_data("assets/cargo-toml/syntax.txt",
            "assets/cargo-toml/test2.txt");
        let _data = load_syntax_data("assets/cargo-toml/syntax.txt",
            "assets/cargo-toml/test3.txt");
        // json::print(&_data);
    }

    #[test]
    fn update_is_json() {
        let _ = load_syntax_data("assets/json/syntax.txt",
            "assets/update/test.txt");
        let _ = load_syntax_data("assets/json/syntax.txt",
            "assets/update/test2.txt");
        let _ = load_syntax_data("assets/json/syntax.txt",
            "assets/update/test3.txt");
    }

    #[test]
    fn update() {
        let _data = load_syntax_data("assets/update/syntax.txt",
            "assets/update/test.txt");
        let _data = load_syntax_data("assets/update/syntax.txt",
            "assets/update/test2.txt");
        let _data = load_syntax_data("assets/update/syntax.txt",
            "assets/update/test3.txt");
        // json::print(&_data);
    }

    /*
    #[test]
    fn from_url() {
        use super::*;

        let data = load_text_file_from_url("https://raw.githubusercontent.com/PistonDevelopers/piston/master/src/input/Cargo.toml");
        assert!(data.is_ok());
    }
    */
}
