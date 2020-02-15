pub mod parser;
pub mod generator;

use std::fs;

fn main() {
    let text = fs::read_to_string("./ace/main.ace").unwrap();
    parser::grammar::parse(&text);
}
