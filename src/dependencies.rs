use range::Range;
use piston_meta::MetaData;

use std::rc::Rc;

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
