//! Generate update information from dependency information.

use std::rc::Rc;
use std::collections::HashMap;

use semver::Version;

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
            let version = try!(Version::parse(&dep.version)
                .map_err(|_| format!("Could not parse version `{}` for `{}`",
                    &dep.version, &dep.name)));
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

    Ok(String::from(""))
}
