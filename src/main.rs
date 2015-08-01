extern crate piston_meta;

use piston_meta::*;

fn main() {

}

#[cfg(test)]
mod tests {
    use piston_meta::*;

    #[test]
    fn extract_is_json() {
        let _ = load_syntax_data("assets/json/syntax.txt",
            "assets/extract/test.txt");
    }

    #[test]
    fn extract() {
        let _data = load_syntax_data("assets/extract/syntax.txt",
            "assets/extract/test.txt");
        // json::print(&_data);
    }
}
