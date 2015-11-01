//! Dependency info.

use range::Range;
use piston_meta::{ json, MetaData };
use piston_meta::bootstrap::Convert;

use std::sync::Arc;
use std::io::{ self, Write };

/// Writes the dependency info.
pub fn write<W: Write>(
    package_data: &[Package],
    w: &mut W
) -> Result<(), io::Error> {
    try!(writeln!(w, "{{"));
    let n0 = package_data.len();
    for (i0, package) in package_data.iter().enumerate() {
        // Package name.
        try!(write!(w, " "));
        try!(json::write_string(w, &package.name));
        try!(writeln!(w, ": {{"));

        // Version.
        try!(write!(w, "  \"version\": "));
        try!(json::write_string(w, &package.version));
        try!(writeln!(w, ","));

        // Dependencies.
        try!(writeln!(w, "  \"dependencies\": {{"));
        let n1 = package.dependencies.len();
        for (i1, dependency) in package.dependencies.iter().enumerate() {
            try!(write!(w, "   "));
            try!(json::write_string(w, &dependency.name));
            try!(writeln!(w, ": {{"));
            // Version.
            try!(write!(w, "    \"version\": "));
            try!(json::write_string(w, &dependency.version));
            try!(writeln!(w, ""));
            try!(write!(w, "   }}"));
            if i1 + 1 != n1 {
                try!(writeln!(w, ","));
            } else {
                try!(writeln!(w, ""));
            }
        }
        try!(writeln!(w, "  }},"));

        // Dev dependencies.
        try!(writeln!(w, "  \"dev-dependencies\": {{"));
        let n1 = package.dev_dependencies.len();
        for (i1, dependency) in package.dev_dependencies.iter().enumerate() {
            try!(write!(w, "   "));
            try!(json::write_string(w, &dependency.name));
            try!(writeln!(w, ": {{"));
            // Version.
            try!(write!(w, "    \"version\": "));
            try!(json::write_string(w, &dependency.version));
            try!(writeln!(w, ""));
            try!(write!(w, "   }}"));
            if i1 + 1 != n1 {
                try!(writeln!(w, ","));
            } else {
                try!(writeln!(w, ""));
            }
        }
        try!(writeln!(w, "  }}"));

        // End package.
        try!(write!(w, " }}"));
        if i0 + 1 != n0 {
            try!(writeln!(w, ","));
        } else {
            try!(writeln!(w, ""));
        }
    }
    try!(writeln!(w, "}}"));
    Ok(())
}

/// Converts from meta data to dependency information.
pub fn convert(
    data: &[Range<MetaData>],
    ignored: &mut Vec<Range>
) -> Result<Vec<Package>, ()> {
    let mut convert = Convert::new(data);
    let mut res = vec![];
    loop {
        if let Ok((range, package)) = Package::from_meta_data(convert, ignored) {
            convert.update(range);
            res.push(package);
        } else if convert.remaining_data_len() > 0 {
            return Err(());
        } else {
            break;
        }
    }
    Ok(res)
}

/// Stores package information.
pub struct Package {
    /// The package name.
    pub name: Arc<String>,
    /// The version.
    pub version: Arc<String>,
    /// Dependencies.
    pub dependencies: Vec<Dependency>,
    /// Dev dependencies.
    pub dev_dependencies: Vec<Dependency>,
}

impl Package {
    /// Converts from meta data.
    pub fn from_meta_data(
        mut convert: Convert,
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Package), ()> {
        let start = convert.clone();
        let node = "package";
        let start_range = try!(convert.start_node(node));
        convert.update(start_range);

        let mut name: Option<Arc<String>> = None;
        let mut version: Option<Arc<String>> = None;
        let mut dependencies = vec![];
        let mut dev_dependencies = vec![];
        loop {
            if let Ok(range) = convert.end_node(node) {
                convert.update(range);
                break;
            } else if let Ok((range, val)) = convert.meta_string("name") {
                convert.update(range);
                name = Some(val);
            } else if let Ok((range, val)) = convert.meta_string("version") {
                convert.update(range);
                version = Some(val);
            } else if let Ok((range, dependency)) = Dependency::from_meta_data(
                "dependency", convert, ignored
            ) {
                convert.update(range);
                dependencies.push(dependency);
            } else if let Ok((range, dev_dependency)) = Dependency::from_meta_data(
                "dev_dependency", convert, ignored
            ) {
                convert.update(range);
                dev_dependencies.push(dev_dependency);
            } else {
                let range = convert.ignore();
                convert.update(range);
                ignored.push(range);
            }
        }

        let name = try!(name.ok_or(()));
        let version = try!(version.ok_or(()));
        Ok((convert.subtract(start), Package {
            name: name,
            version: version,
            dependencies: dependencies,
            dev_dependencies: dev_dependencies,
        }))
    }
}

/// Stores dependency information.
pub struct Dependency {
    /// The package name.
    pub name: Arc<String>,
    /// The semver version of the library.
    pub version: Arc<String>,
}

impl Dependency {
    /// Converts from meta data.
    pub fn from_meta_data(
        node: &str,
        mut convert: Convert,
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Dependency), ()> {
        let start = convert.clone();
        let start_range = try!(convert.start_node(node));
        convert.update(start_range);

        let mut name: Option<Arc<String>> = None;
        let mut version: Option<Arc<String>> = None;
        loop {
            if let Ok(range) = convert.end_node(node) {
                convert.update(range);
                break;
            } else if let Ok((range, val)) = convert.meta_string("name") {
                convert.update(range);
                name = Some(val);
            } else if let Ok((range, val)) = convert.meta_string("version") {
                convert.update(range);
                version = Some(val);
            } else {
                let range = convert.ignore();
                convert.update(range);
                ignored.push(range);
            }
        }

        let name = try!(name.ok_or(()));
        let version = try!(version.ok_or(()));
        Ok((convert.subtract(start), Dependency {
            name: name,
            version: version
        }))
    }
}
