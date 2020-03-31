extern crate paragraph_sort;
use paragraph_sort::paragraph_sort;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("Failed reading from stdin");

    let result = paragraph_sort(&input);

    stdout()
        .write_all(&result.into_bytes())
        .expect("Writing to stdout failed!");
}
