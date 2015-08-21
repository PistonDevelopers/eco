#![deny(missing_docs)]

//! A tool for reasoning about breaking changes in Rust ecosystems

extern crate range;
extern crate piston_meta;
extern crate hyper;
extern crate semver;

pub mod extract;
pub mod update;
pub mod dependencies;

fn main() {
    use std::io::Read;
    use std::fs::File;

    // Load extract info from file.
    let mut extract_info_file = File::open("assets/extract/test.txt").unwrap();
    let mut extract_info = String::new();
    extract_info_file.read_to_string(&mut extract_info).unwrap();

    let res = extract::extract_dependency_info_from(&extract_info).unwrap();
    println!("{}", res);
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
        let _data = load_syntax_data("assets/dependencies/syntax.txt",
            "assets/dependencies/test2.txt");
        // json::print(&_data);
    }

    #[test]
    fn cargo_toml() {
        let _data = load_syntax_data("assets/cargo-toml/syntax.txt",
            "assets/cargo-toml/test.txt");
        // json::print(&_data);
    }

    #[test]
    fn update_is_json() {
        let _ = load_syntax_data("assets/json/syntax.txt",
            "assets/update/test.txt");
    }

    #[test]
    fn update() {
        let _data = load_syntax_data("assets/update/syntax.txt",
            "assets/update/test.txt");
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
