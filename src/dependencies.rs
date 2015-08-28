//! Dependency info.

use range::Range;
use piston_meta::{ json, MetaData };

use std::rc::Rc;
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
    mut data: &[(Range, MetaData)],
    ignored: &mut Vec<Range>
) -> Result<Vec<Package>, ()> {
    use piston_meta::bootstrap::update;

    let mut offset = 0;
    let mut res = vec![];
    loop {
        if let Ok((range, package)) = Package::from_meta_data(data, offset, ignored) {
            update(range, &mut data, &mut offset);
            res.push(package);
        } else if offset < data.len() {
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
    pub name: Rc<String>,
    /// The version.
    pub version: Rc<String>,
    /// Dependencies.
    pub dependencies: Vec<Dependency>,
}

impl Package {
    /// Converts from meta data.
    pub fn from_meta_data(
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Package), ()> {
        use piston_meta::bootstrap::*;

        let start_offset = offset;
        let node = "package";
        let start_range = try!(start_node(node, data, offset));
        update(start_range, &mut data, &mut offset);

        let mut name: Option<Rc<String>> = None;
        let mut version: Option<Rc<String>> = None;
        let mut dependencies = vec![];
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_string("name", data, offset) {
                update(range, &mut data, &mut offset);
                name = Some(val);
            } else if let Ok((range, val)) = meta_string("version", data, offset) {
                update(range, &mut data, &mut offset);
                version = Some(val);
            } else if let Ok((range, dependency)) = Dependency::from_meta_data(data, offset, ignored) {
                update(range, &mut data, &mut offset);
                dependencies.push(dependency);
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }

        let name = try!(name.ok_or(()));
        let version = try!(version.ok_or(()));
        Ok((Range::new(start_offset, offset - start_offset), Package {
            name: name,
            version: version,
            dependencies: dependencies,
        }))
    }
}

/// Stores dependency information.
pub struct Dependency {
    /// The package name.
    pub name: Rc<String>,
    /// The semver version of the library.
    pub version: Rc<String>,
}

impl Dependency {
    /// Converts from meta data.
    pub fn from_meta_data(
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        ignored: &mut Vec<Range>
    ) -> Result<(Range, Dependency), ()> {
        use piston_meta::bootstrap::*;

        let start_offset = offset;
        let node = "dependency";
        let start_range = try!(start_node(node, data, offset));
        update(start_range, &mut data, &mut offset);

        let mut name: Option<Rc<String>> = None;
        let mut version: Option<Rc<String>> = None;
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_string("name", data, offset) {
                update(range, &mut data, &mut offset);
                name = Some(val);
            } else if let Ok((range, val)) = meta_string("version", data, offset) {
                update(range, &mut data, &mut offset);
                version = Some(val);
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }

        let name = try!(name.ok_or(()));
        let version = try!(version.ok_or(()));
        Ok((Range::new(start_offset, offset - start_offset), Dependency {
            name: name,
            version: version
        }))
    }
}
