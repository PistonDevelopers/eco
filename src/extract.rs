//! Extract dependency information from extract info.
//!
//! ### Extract info
//!
//! This is a JSON format with meta syntax `./assets/extract/syntax.txt`.
//!
//! It is used for collecting dependency data from raw Cargo.toml files.
//!
//! Example:
//!
//! ```json
//! {
//!    "pistoncore-input": {
//!        "url": "https://raw.githubusercontent.com/PistonDevelopers/piston/master/src/input/Cargo.toml",
//!        "ignore-version": "0.7.0"
//!    },
//!
//!    "pistoncore-window": {
//!        "url": "https://raw.githubusercontent.com/PistonDevelopers/piston/master/src/window/Cargo.toml"
//!    },
//!
//!    "pistoncore-event_loop": {
//!        "url": "https://raw.githubusercontent.com/PistonDevelopers/piston/master/src/event_loop/Cargo.toml"
//!    },
//!
//!    "piston": {
//!        "url": "https://raw.githubusercontent.com/PistonDevelopers/piston/master/Cargo.toml"
//!    }
//! }
//! ```
//!
//! Fields:
//!
//! - url (the url to the raw Cargo.toml data)
//! - ignore-version (don't update projects using this version)
//! - override-version (replace version extracted from Cargo.toml)
//! - ignore (ignore update of specific dependency version)
//!
//! ### Ignore version
//!
//! The ignore-version field is used to delay updates, to reduce frequency
//! of breaking changes, or put them on hold until some work is done.
//!
//! Example:
//!
//! ```text
//! A (uses B 0.7.0) -> B (0.8.0)
//! ```
//!
//! A new version of library B is available, but some work might be needed in A
//! before releasing a new version. By listing "0.8.0" in the ignore-version
//! field, there will be no recommended update for A. This will also avoid
//! further updates for libraries depending on A triggered by this version.
//!
//! This might cause unsoundness (see top level documentation) when done to
//! a library that is not at the bottom of the dependency graph.
//!
//! The rule used for ignoring version is: If the package has this version,
//! then filter it from dependency info.
//!
//! ### Override version
//!
//! The override-version field replaces the version extracted from
//! Cargo.toml with another version.
//!
//! For example, a new library is published but the maintainer forgot
//! to merge the changes into master.
//!
//! ### Ignore
//!
//! The ignore field specifies a collection of dependency versions to ignore.
//! This allows precise control over which dependencies to update.
//!
//! Example:
//!
//! ```text
//! {
//!     "piston2d-opengl_graphics": {
//!         "url": "https://raw.githubusercontent.com/PistonDevelopers/opengl_graphics/master/Cargo.toml",
//!         "ignore": {
//!             "piston-shaders_graphics2d": "0.2.1"
//!         }
//!     }
//! }
//! ```
//!
//! This might cause unsoundess (see top level documentation) when some of
//! the interface of the dependency is exposed in the interface of the library.
//!
//! The rule used for ignoring dependencies is:
//! If there is a recommended update to this version,
//! then filter it from update info.

use piston_meta::MetaData;
use piston_meta::bootstrap::Convert;
use dependencies::{self, Package};
use range::Range;
use std::sync::Arc;

/// Stores extract information.
pub struct Extract {
    /// The package name.
    pub package: Arc<String>,
    /// The url of the Cargo.toml.
    pub url: Arc<String>,
    /// Ignore updates to projects using this version.
    pub ignore_version: Option<Arc<String>>,
    /// Override package version.
    pub override_version: Option<Arc<String>>,
    /// Ignore specific dependency versions.
    pub ignore: Vec<(Arc<String>, Arc<String>)>,
}

impl Extract {
    /// Converts from meta data.
    pub fn from_meta_data(
        mut convert: Convert,
        ignored: &mut Vec<Range>,
    ) -> Result<(Range, Extract), ()> {
        let start = convert.clone();
        let node = "library";
        let start_range = try!(convert.start_node(node));
        convert.update(start_range);

        let mut package: Option<Arc<String>> = None;
        let mut url: Option<Arc<String>> = None;
        let mut ignore_version: Option<Arc<String>> = None;
        let mut override_version: Option<Arc<String>> = None;
        let mut ignore: Vec<(Arc<String>, Arc<String>)> = vec![];
        loop {
            if let Ok(range) = convert.end_node(node) {
                convert.update(range);
                break;
            } else if let Ok((range, val)) = convert.meta_string("package") {
                convert.update(range);
                package = Some(val);
            } else if let Ok((range, val)) = convert.meta_string("url") {
                convert.update(range);
                url = Some(val);
            } else if let Ok((range, val)) = convert.meta_string("ignore_version") {
                convert.update(range);
                ignore_version = Some(val);
            } else if let Ok((range, val)) = convert.meta_string("override_version") {
                convert.update(range);
                override_version = Some(val);
            } else if let Ok((range, val)) = ignore_from_meta_data(convert, ignored) {
                convert.update(range);
                ignore = val;
            } else {
                let range = convert.ignore();
                convert.update(range);
                ignored.push(range);
            }
        }

        let package = try!(package.ok_or(()));
        let url = try!(url.ok_or(()));
        Ok((
            convert.subtract(start),
            Extract {
                package: package,
                url: url,
                ignore_version: ignore_version,
                override_version: override_version,
                ignore: ignore,
            },
        ))
    }
}

fn ignore_from_meta_data(
    mut convert: Convert,
    ignored: &mut Vec<Range>,
) -> Result<(Range, Vec<(Arc<String>, Arc<String>)>), ()> {
    let start = convert.clone();
    let node = "ignore";
    let start_range = try!(convert.start_node(node));
    convert.update(start_range);

    let mut res: Vec<(Arc<String>, Arc<String>)> = vec![];
    loop {
        if let Ok(range) = convert.end_node(node) {
            convert.update(range);
            break;
        } else if let Ok((range, val)) = dependency_from_meta_data(convert, ignored) {
            convert.update(range);
            res.push(val);
        } else {
            let range = convert.ignore();
            convert.update(range);
            ignored.push(range);
        }
    }

    Ok((convert.subtract(start), res))
}

fn dependency_from_meta_data(
    mut convert: Convert,
    ignored: &mut Vec<Range>,
) -> Result<(Range, (Arc<String>, Arc<String>)), ()> {
    let start = convert.clone();
    let node = "dependency";
    let start_range = try!(convert.start_node(node));
    convert.update(start_range);

    let mut package: Option<Arc<String>> = None;
    let mut version: Option<Arc<String>> = None;
    loop {
        if let Ok(range) = convert.end_node(node) {
            convert.update(range);
            break;
        } else if let Ok((range, val)) = convert.meta_string("package") {
            convert.update(range);
            package = Some(val);
        } else if let Ok((range, val)) = convert.meta_string("version") {
            convert.update(range);
            version = Some(val);
        } else {
            let range = convert.ignore();
            convert.update(range);
            ignored.push(range);
        }
    }

    let package = try!(package.ok_or(()));
    let version = try!(version.ok_or(()));
    Ok((convert.subtract(start), (package, version)))
}

/// Loads a text file from url.
pub fn load_text_file_from_url(url: &str) -> Result<String, String> {
    use hyper::client::Client;
    use hyper::Url;
    use hyper::status::StatusCode;
    use std::io::Read;

    let url_address = try!(Url::parse(url).map_err(|e| format!("Error parsing url: {}", e)));
    let client = Client::new();
    let request = client.get(url_address);
    let mut response = try!(request.send().map_err(|e| format!(
        "Error fetching file over http {}: {}",
        url,
        e.to_string()
    )));
    if response.status == StatusCode::Ok {
        let mut data = String::new();
        try!(response.read_to_string(&mut data).map_err(|e| format!(
            "Error fetching file over http {}: {}",
            url,
            e.to_string()
        )));
        Ok(data)
    } else {
        Err(format!(
            "Error fetching file over http {}: {}",
            url, response.status
        ))
    }
}

/// Converts meta data into extract info.
pub fn convert_extract_info(
    data: &[Range<MetaData>],
    ignored: &mut Vec<Range>,
) -> Result<Vec<Extract>, ()> {
    use piston_meta::bootstrap::*;

    let mut list = vec![];
    let mut convert = Convert::new(data);
    loop {
        if let Ok((range, extract)) = Extract::from_meta_data(convert, ignored) {
            convert.update(range);
            list.push(extract);
        } else if convert.remaining_data_len() > 0 {
            return Err(());
        } else {
            break;
        }
    }
    Ok(list)
}

/// Converts meta data into Cargo.toml information.
pub fn convert_cargo_toml(
    data: &[Range<MetaData>],
    ignored: &mut Vec<Range>,
) -> Result<Package, ()> {
    let (_, package) = try!(Package::from_meta_data(Convert::new(data), ignored));
    Ok((package))
}

/// Extracts dependency info.
pub fn extract_dependency_info_from(extract_info: &str) -> Result<String, String> {
    use std::sync::Mutex;
    use std::thread;
    use piston_meta::*;

    let extract_meta_syntax = include_str!("../assets/extract/syntax.txt");
    let extract_meta_rules = stderr_unwrap(extract_meta_syntax, syntax(extract_meta_syntax));
    let mut extract_data = vec![];
    stderr_unwrap(
        extract_info,
        parse(&extract_meta_rules, extract_info, &mut extract_data),
    );

    let mut ignored = vec![];
    let list = try!(
        convert_extract_info(&extract_data, &mut ignored)
            .map_err(|_| String::from("Could not convert extract data"))
    );

    // Stores package and dependency information extracted from Cargo.toml.
    let package_data = Arc::new(Mutex::new(vec![]));

    // Extract information.
    let cargo_toml_syntax = include_str!("../assets/cargo-toml/syntax.txt");
    let cargo_toml_rules = Arc::new(stderr_unwrap(cargo_toml_syntax, syntax(cargo_toml_syntax)));
    let mut handles = vec![];
    for extract in list.into_iter() {
        let cargo_toml_rules = cargo_toml_rules.clone();
        let package_data = package_data.clone();
        handles.push(thread::spawn(move || {
            let config = try!(load_text_file_from_url(&extract.url));
            let mut cargo_toml_data = vec![];
            match parse(&cargo_toml_rules, &config, &mut cargo_toml_data) {
                Ok(val) => val,
                Err(range_err) => {
                    let mut w: Vec<u8> = vec![];
                    ParseErrorHandler::new(&config)
                        .write(&mut w, range_err)
                        .unwrap();
                    return Err(format!(
                        "{}: Syntax error in Cargo.toml for url `{}`\n{}",
                        &extract.package,
                        &extract.url,
                        &String::from_utf8(w).unwrap()
                    ));
                }
            };

            let mut ignored = vec![];
            let mut package = try!(convert_cargo_toml(&cargo_toml_data, &mut ignored).map_err(
                |_| format!(
                    "Could not convert Cargo.toml data for url `{}`",
                    &extract.url
                )
            ));
            if extract.package != package.name {
                return Err(format!(
                    "Wrong Cargo.toml: `{}` does not match `{}`",
                    extract.package, package.name
                ));
            }
            if let Some(ref ignore_version) = extract.ignore_version {
                if ignore_version == &package.version {
                    return Ok(());
                }
            }
            if let Some(ref override_version) = extract.override_version {
                package.version = override_version.clone();
            }
            for &(ref name, ref version) in &extract.ignore {
                for dep in &mut package.dependencies {
                    if &**dep.name == &**name {
                        dep.ignore_version = Some(version.clone());
                    }
                }
                for dep in &mut package.dev_dependencies {
                    if &**dep.name == &**name {
                        dep.ignore_version = Some(version.clone());
                    }
                }
            }
            package_data.lock().unwrap().push(package);
            Ok(())
        }))
    }
    for handle in handles.into_iter() {
        try!(handle.join().unwrap().map_err(|e| e));
    }

    let mut res: Vec<u8> = vec![];
    dependencies::write(&package_data.lock().unwrap(), &mut res).unwrap();

    let res = try!(String::from_utf8(res).map_err(|e| format!("UTF8 error: {}", e)));

    Ok(res)
}
