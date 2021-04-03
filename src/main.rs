use cedict_parser::{parse_cedict, CedictEntry};

pub mod lib;

fn main() {
    let contents = std::fs::read_to_string("cedict_ts.u8").unwrap();

    let entries: Vec<CedictEntry> = parse_cedict(&contents);

    println!("Number of entries: {}", entries.len());
}
