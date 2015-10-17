//! Generate todo list for each package.

/// Generates a todo list from extract info.
/// This can be used in Github markdown.
///
/// ```ignore
/// - [ ] foo
/// - [ ] bar
/// - [ ] baz
/// ```
pub fn todo_from_extract_info(extract_info: &str) -> Result<String, String> {
    use std::io::Write;
    use piston_meta::*;
    use extract::*;

    let extract_meta_syntax = include_str!("../assets/extract/syntax.txt");
    let extract_meta_rules = stderr_unwrap(extract_meta_syntax,
        syntax(extract_meta_syntax));
    let mut extract_data = vec![];
    stderr_unwrap(extract_info,
        parse(&extract_meta_rules, extract_info, &mut extract_data));

    let mut ignored = vec![];
    let list = try!(convert_extract_info(&extract_data, &mut ignored)
        .map_err(|_| String::from("Could not convert extract data")));

    let mut res: Vec<u8> = vec![];
    for package in &list {
        writeln!(&mut res, "- [ ] {}", &package.package).unwrap();
    }
    Ok(String::from_utf8(res).unwrap())
}
