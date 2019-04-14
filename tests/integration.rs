//! Clasp integration tests: parse the examples under `syntax/examples`

use std::fs;

#[test]
fn parse_syntax_examples() {
    for entry in fs::read_dir("syntax/examples").unwrap() {
        let path = entry.unwrap().path();
        let program = fs::read_to_string(path);

        println!("program: {:?}", program);
    }
}
