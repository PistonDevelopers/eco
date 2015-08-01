extern crate piston_meta;

use piston_meta::*;

fn main() {

}

#[cfg(test)]
mod tests {
    use piston_meta::*;

    #[test]
    fn extract() {
        let data = load_syntax_data("assets/extract/syntax.txt",
            "assets/extract/test.txt");
        json::print(&data);
    }
}
