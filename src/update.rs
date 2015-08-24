//! Generate update information from dependency information.

use std::rc::Rc;
use std::collections::HashMap;
use std::io::Write;

use semver::{ self, Version };

use dependencies;

/// Stores old and new version.
pub struct Bump {
    /// The old version.
    pub old: Version,
    /// The new version.
    pub new: Version,
}

/// Stores update info for dependency.
pub struct Dependency {
    /// The library name.
    pub name: Rc<String>,
    /// Bump info.
    pub bump: Bump,
}

/// Stores information about a package.
pub struct Package {
    /// The package name.
    pub name: Rc<String>,
    /// The order to update library.
    pub order: u32,
    /// Update information.
    pub bump: Bump,
    /// The dependencies.
    pub dependencies: Vec<Dependency>,
}

/// Generates update info.
pub fn generate_update_info_from(dependency_info: &str) -> Result<String, String> {
    use piston_meta::*;
    use std::iter::FromIterator;

    type PackageIndex = usize;
    type Depth = u32;

    fn depth_of(
        package_index: PackageIndex,
        package_indices: &HashMap<Rc<String>, PackageIndex>,
        depths: &mut HashMap<PackageIndex, Depth>,
        dependencies_data: &[dependencies::Package]
    ) -> Depth {
        let package = &dependencies_data[package_index];
        let d = depths.get(&package_index).map(|d| *d);
        match d {
            None => {
                // The depth of a package equals maximum depth of the dependencies + 1.
                let new_depth: Depth = package.dependencies.iter().map(|dep| {
                        package_indices.get(&dep.name).map(|&p| {
                                depth_of(p, package_indices, depths, dependencies_data)
                            }).unwrap_or(0)
                    }).max().unwrap_or(0) + 1;
                depths.insert(package_index, new_depth);
                new_depth
            }
            Some(x) => x
        }
    }

    // Tries appending zero to version to make it parse.
    fn parse_version(text: &str) -> Result<Version, semver::ParseError> {
        // Ignore `>=`.
        let text = if text.starts_with(">=") {
            &text[2..]
        } else {
            text
        };
        match Version::parse(text) {
            Err(_) => {
                let append_zero = format!("{}.0", text);
                Version::parse(&append_zero)
            }
            x => x
        }
    }

    // Whether the version should be ignored.
    fn ignore_version(text: &str) -> bool {
        text == "*"
    }

    // Returns true if two different versions means a breaking change.
    fn breaks(a: &Version, b: &Version) -> bool {
        if a.major != b.major { true }
        else if a.major != 0 { false }
        else if a.minor != b.minor { true }
        else if a.minor != 0 { false }
        else if a.patch != b.patch { true }
        else { false }
    }

    // Increment first non-zero number.
    fn increment_version(version: &mut Version) {
        if version.major != 0 { version.increment_major(); }
        else if version.minor != 0 { version.increment_minor(); }
        else { version.increment_patch(); }
    }

    // Parse and convert to dependencies data.
    let dependencies_meta_syntax = include_str!("../assets/dependencies/syntax.txt");
    let dependencies_meta_rules = stderr_unwrap(dependencies_meta_syntax,
        syntax(dependencies_meta_syntax));
    let dependency_info = stderr_unwrap(dependency_info,
        parse(&dependencies_meta_rules, dependency_info));
    let mut ignored = vec![];
    let dependencies_data = try!(dependencies::convert(&dependency_info, &mut ignored)
        .map_err(|_| String::from("Could not convert dependency info")));

    // Stores the package indices using package name as key.
    let package_indices: HashMap<Rc<String>, PackageIndex> =
        HashMap::from_iter(dependencies_data.iter().enumerate().map(
            |(i, p)| {
                (p.name.clone(), i)
            }));

    // Store the depths of libraries.
    let mut depths: HashMap<PackageIndex, Depth> = HashMap::new();
    for i in 0 .. dependencies_data.len() {
        let _depth = depth_of(i, &package_indices, &mut depths, &dependencies_data);
    }

    let mut new_versions: HashMap<Rc<String>, Version> = HashMap::new();
    for package in &dependencies_data {
        // Get latest version used by any dependency.
        for dep in &package.dependencies {
            if ignore_version(&dep.version) { continue; }

            let version = try!(parse_version(&dep.version)
                .map_err(|_| format!("Could not parse version `{}` for dependency `{}` in `{}`",
                    &dep.version, &dep.name, &package.name)));
            let v = new_versions.get(&dep.name).map(|v| v.clone());
            match v {
                None => {
                    new_versions.insert(dep.name.clone(), version);
                }
                Some(v) => {
                    if v < version {
                        new_versions.insert(dep.name.clone(), version);
                    }
                }
            }
        }
    }

    // Overwrite the versions used by packages.
    for package in &dependencies_data {
        let version = try!(Version::parse(&package.version)
            .map_err(|_| format!("Could not parse version `{}` for `{}`",
                &package.version, &package.name)));
        new_versions.insert(package.name.clone(), version);
    }

    // Create list of sorted package indices by depth.
    let mut sorted_depths: Vec<_> = depths.iter().collect();
    sorted_depths.sort_by(|&(_, da), &(_, db)| da.cmp(db));

    // Stores the update info.
    let mut update_packages: Vec<Package> = vec![];

    for (&package_index, &order) in sorted_depths {
        let package = &dependencies_data[package_index];
        let mut update_dependencies = vec![];

        // Find dependencies that needs update.
        for dep in &package.dependencies {
            if ignore_version(&dep.version) { continue; }
            // Do not generate update info for dependencies starting with `>=`.
            if dep.version.starts_with(">=") { continue; }

            let old_version = try!(parse_version(&dep.version)
                .map_err(|_| format!("Could not parse version `{}` for dependency `{}` in `{}`",
                    &dep.version, &dep.name, &package.name)));
            let new_version = new_versions.get(&dep.name).unwrap();
            if breaks(new_version, &old_version) {
                update_dependencies.push(Dependency {
                        name: dep.name.clone(),
                        bump: Bump {
                            old: old_version,
                            new: new_version.clone(),
                        }
                    });
            }
        }

        // If any dependency needs update, then the package needs update.
        if update_dependencies.len() > 0 {
            let old_version = try!(Version::parse(&package.version)
                .map_err(|_| format!("Could not parse version `{}` for `{}`",
                    &package.version, &package.name)));
            let new_version = new_versions.get_mut(&package.name).unwrap();
            if *new_version == old_version {
                increment_version(new_version);
            }
            update_packages.push(Package {
                    name: package.name.clone(),
                    order: order,
                    bump: Bump {
                        old: old_version,
                        new: new_version.clone(),
                    },
                    dependencies: update_dependencies,
                });
        }
    }

    let mut w: Vec<u8> = vec![];
    writeln!(w, "{{").unwrap();
    let n0 = update_packages.len();
    for (i0, update_package) in update_packages.iter().enumerate() {
        write!(w, "  ").unwrap();
        json::write_string(&mut w, &update_package.name).unwrap();
        writeln!(w, ": {{").unwrap();
        writeln!(w, "    \"order\": {},", update_package.order).unwrap();

        // Bump package.
        writeln!(w, "    \"bump\": {{").unwrap();
        write!(w, "      \"old\": ").unwrap();
        json::write_string(&mut w, &format!("{}", update_package.bump.old)).unwrap();
        writeln!(w, ",").unwrap();
        write!(w, "      \"new\": ").unwrap();
        json::write_string(&mut w, &format!("{}", update_package.bump.new)).unwrap();
        writeln!(w, "").unwrap();
        writeln!(w, "    }},").unwrap();

        // Dependencies.
        writeln!(w, "    \"dependencies\": {{").unwrap();
        let n1 = update_package.dependencies.len();
        for (i1, dep) in update_package.dependencies.iter().enumerate() {
            write!(w, "      ").unwrap();
            json::write_string(&mut w, &dep.name).unwrap();
            writeln!(w, ": {{").unwrap();
            writeln!(w, "        \"bump\": {{").unwrap();
            write!(w, "          \"old\": ").unwrap();
            json::write_string(&mut w, &format!("{}", dep.bump.old)).unwrap();
            writeln!(w, ",").unwrap();
            write!(w, "          \"new\": ").unwrap();
            json::write_string(&mut w, &format!("{}", dep.bump.new)).unwrap();
            writeln!(w, "").unwrap();
            writeln!(w, "        }}").unwrap();
            write!(w, "      }}").unwrap();
            if i1 + 1 < n1 {
                writeln!(w, ",").unwrap();
            } else {
                writeln!(w, "").unwrap();
            }
        }
        writeln!(w, "    }}").unwrap();

        write!(w, "  }}").unwrap();
        if i0 + 1 < n0 {
            writeln!(w, ",").unwrap();
        } else {
            writeln!(w, "").unwrap();
        }
    }
    writeln!(w, "}}").unwrap();

    Ok(String::from_utf8(w).unwrap())
}
