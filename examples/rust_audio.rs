extern crate eco;

fn main() {
    use std::io::Read;
    use std::fs::File;

    // Load extract info from file.
    let mut extract_info_file = File::open("assets/extract/rust_audio.txt").unwrap();
    let mut extract_info = String::new();
    extract_info_file.read_to_string(&mut extract_info).unwrap();

    let dependency_info = eco::extract::extract_dependency_info_from(&extract_info).unwrap();
    let update_info = eco::update::generate_update_info_from(&dependency_info).unwrap();
    println!("{}", update_info);
}
