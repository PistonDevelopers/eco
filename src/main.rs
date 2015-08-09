#![deny(missing_docs)]

//! A tool for reasoning about breaking changes in Rust ecosystems

extern crate piston_meta;

use std::rc::Rc;

/// Stores extract information.
pub struct Extract {
    /// The url of the Cargo.toml.
    pub url: String,
    /// Whether to override the library version to simulate breaking change.
    pub override_version: Option<String>,
}

/// Stores dependency information.
pub struct Dependency {
    /// The package name.
    pub name: Rc<String>,
    /// The semver version of the library.
    pub version: String,
}

fn main() {

}

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
        // json::print(&_data);
    }

    #[test]
    fn dependencies_is_json() {
        let _ = load_syntax_data("assets/json/syntax.txt",
            "assets/dependencies/test.txt");
    }

    #[test]
    fn dependencies() {
        let _data = load_syntax_data("assets/dependencies/syntax.txt",
            "assets/dependencies/test.txt");
        // json::print(&_data);
    }
}
