use std::ffi::OsString;

use cedict_parser::{parse_cedict, CedictEntry};

pub mod lib;

fn main() {
    let args: Vec<OsString> = std::env::args_os().collect();

    let cedict_file_path = args.get(1).expect("You need to pass a file path!");

    let contents = std::fs::read_to_string(cedict_file_path).expect("Failed to read file!");

    let entries: Vec<CedictEntry> = parse_cedict(&contents);

    println!("Number of entries: {}", entries.len());
}
