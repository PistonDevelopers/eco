extern crate eco;

fn main() {
    use std::io::Read;
    use std::fs::File;

    // Load extract info from file.
    let mut extract_info_file = File::open("assets/extract/piston.txt").unwrap();
    let mut extract_info = String::new();
    extract_info_file.read_to_string(&mut extract_info).unwrap();

    let todo = eco::todo::todo_from_extract_info(&extract_info).unwrap();
    println!("{}", todo);
}
