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

    fn depth_of(
        package: &dependencies::Package,
        depth: &mut HashMap<Rc<String>, u32>,
        dependencies_data: &[dependencies::Package]
    ) -> u32 {
        let d = depth.get(&package.name).map(|d| *d);
        match d {
            None => {
                // The depth of a package equals maximum depth of the dependencies + 1.
                let new_depth: u32 = package.dependencies.iter().map(|dep| {
                        // Get depth of dependency.
                        // If none of the dependencies are listed, treat it as zero.
                        dependencies_data.iter().filter(|p| {
                                p.name == dep.name
                            }).next().map(|p| {
                                depth_of(p, depth, dependencies_data)
                            }).unwrap_or(0)
                    }).max().unwrap_or(0) + 1;
                depth.insert(package.name.clone(), new_depth);
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

    let mut depth: HashMap<Rc<String>, u32> = HashMap::new();
    for package in &dependencies_data {
        let _depth = depth_of(package, &mut depth, &dependencies_data);
    }

    Ok(String::from(""))
}
