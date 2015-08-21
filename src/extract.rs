//! Extract dependency information from extract info.

use piston_meta::MetaData;
use dependencies::Package;
use range::Range;
use std::rc::Rc;

/// Stores extract information.
pub struct Extract {
    /// The package name.
    pub package: Rc<String>,
    /// The url of the Cargo.toml.
    pub url: Rc<String>,
    /// Whether to override the library version to simulate breaking change.
    pub override_version: Option<Rc<String>>,
}

impl Extract {
    /// Converts from meta data.
    pub fn from_meta_data(
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Extract), ()> {
        use piston_meta::bootstrap::*;

        let start_offset = offset;
        let node = "library";
        let start_range = try!(start_node(node, data, offset));
        update(start_range, &mut data, &mut offset);

        let mut package: Option<Rc<String>> = None;
        let mut url: Option<Rc<String>> = None;
        let mut override_version: Option<Rc<String>> = None;
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_string("package", data, offset) {
                update(range, &mut data, &mut offset);
                package = Some(val);
            } else if let Ok((range, val)) = meta_string("url", data, offset) {
                update(range, &mut data, &mut offset);
                url = Some(val);
            } else if let Ok((range, val)) = meta_string("override_version", data, offset) {
                update(range, &mut data, &mut offset);
                override_version = Some(val);
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }

        let package = try!(package.ok_or(()));
        let url = try!(url.ok_or(()));
        Ok((Range::new(start_offset, offset - start_offset), Extract {
            package: package,
            url: url,
            override_version: override_version,
        }))
    }
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

/// Converts meta data into extract info.
pub fn convert_extract_info(
    mut data: &[(Range, MetaData)],
    ignored: &mut Vec<Range>
) -> Result<Vec<Extract>, ()> {
    use piston_meta::bootstrap::*;

    let mut list = vec![];
    let mut offset = 0;
    loop {
        if let Ok((range, extract)) = Extract::from_meta_data(data, offset, ignored) {
            update(range, &mut data, &mut offset);
            list.push(extract);
        } else if offset < data.len() {
            return Err(());
        } else {
            break;
        }
    }
    Ok(list)
}

/// Converts meta data into Cargo.toml information.
pub fn convert_cargo_toml(
    data: &[(Range, MetaData)],
    ignored: &mut Vec<Range>
) -> Result<Package, ()> {
    let offset = 0;
    let (_, package) = try!(Package::from_meta_data(data, offset, ignored));
    Ok((package))
}

/// Extracts dependency info.
pub fn extract_dependency_info_from(extract_info: &str) -> Result<String, String> {
    use piston_meta::*;
    use std::io::Write;

    let extract_meta_syntax = include_str!("../assets/extract/syntax.txt");
    let extract_meta_rules = stderr_unwrap(extract_meta_syntax,
        syntax(extract_meta_syntax));
    let extract_data = stderr_unwrap(extract_info,
        parse(&extract_meta_rules, extract_info));

    let mut ignored = vec![];
    let list = try!(convert_extract_info(&extract_data, &mut ignored)
        .map_err(|_| String::from("Could not convert extract data")));

    // Stores package and dependency information extracted from Cargo.toml.
    let mut package_data = vec![];

    // Extract information.
    let cargo_toml_syntax = include_str!("../assets/cargo-toml/syntax.txt");
    let cargo_toml_rules = stderr_unwrap(cargo_toml_syntax,
        syntax(cargo_toml_syntax));
    for extract in &list {
        let config = try!(load_text_file_from_url(&extract.url));
        let cargo_toml_data = stderr_unwrap(&config,
            parse(&cargo_toml_rules, &config));

        let mut ignored = vec![];
        let package = try!(convert_cargo_toml(
            &cargo_toml_data, &mut ignored)
            .map_err(|_| format!("Could not convert Cargo.toml data for url `{}`", &extract.url)));
        package_data.push(package);
    }

    let mut res: Vec<u8> = vec![];
    writeln!(res, "{{").unwrap();
    let n0 = package_data.len();
    for (i0, package) in package_data.iter().enumerate() {
        // Package name.
        write!(res, " ").unwrap();
        json::write_string(&mut res, &package.name).unwrap();
        writeln!(res, ": {{").unwrap();

        // Version.
        write!(res, "  \"version\": ").unwrap();
        json::write_string(&mut res, &package.version).unwrap();
        writeln!(res, ",").unwrap();

        // Dependencies.
        writeln!(res, "  \"dependencies\": {{").unwrap();
        let n1 = package.dependencies.len();
        for (i1, dependency) in package.dependencies.iter().enumerate() {
            write!(res, "   ").unwrap();
            json::write_string(&mut res, &dependency.name).unwrap();
            writeln!(res, ": {{").unwrap();
            // Version.
            write!(res, "    \"version\": ").unwrap();
            json::write_string(&mut res, &dependency.version).unwrap();
            writeln!(res, "").unwrap();
            write!(res, "   }}").unwrap();
            if i1 + 1 != n1 {
                writeln!(res, ",").unwrap();
            } else {
                writeln!(res, "").unwrap();
            }
        }
        writeln!(res, "  }}").unwrap();

        // End package.
        write!(res, " }}").unwrap();
        if i0 + 1 != n0 {
            writeln!(res, ",").unwrap();
        } else {
            writeln!(res, "").unwrap();
        }
    }
    writeln!(res, "}}").unwrap();

    let res = try!(String::from_utf8(res)
        .map_err(|e| format!("UTF8 error: {}", e)));

    Ok(res)
}
