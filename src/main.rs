#![deny(missing_docs)]

//! A tool for reasoning about breaking changes in Rust ecosystems

extern crate piston_meta;
extern crate hyper;

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

/// Loads a text file from url.
pub fn load_text_file_from_url(url: &str) -> Result<String, String> {
    use hyper::client::Client;
    use hyper::{Url};
    use hyper::status::StatusCode;
    use std::io::Read;

    let url_address = try!(Url::parse(url)
        .map_err(|e| format!("Error parsing url: {}", e)));
    let client = Client::new();
    let request = client.get(url_address);
    let mut response = try!(request.send()
        .map_err(|e| format!("Error fetching file over http {}: {}",
            url, e.to_string())));
    if response.status == StatusCode::Ok {
        let mut data = String::new();
        try!(response.read_to_string(&mut data)
            .map_err(|e| format!("Error fetching file over http {}: {}",
            url, e.to_string())));
        Ok(data)
    } else {
        Err(format!("Error fetching file over http {}: {}",
            url, response.status))
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn from_url() {
        let data = load_text_file_from_url("https://raw.githubusercontent.com/PistonDevelopers/piston/master/src/input/Cargo.toml");
        assert!(data.is_ok());
    }
}
