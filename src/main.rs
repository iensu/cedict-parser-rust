fn main() {
    let contents = std::fs::read_to_string("cedict_ts.u8").unwrap();

    let entries: Vec<CedictEntry> = contents
        .lines()
        .filter_map(|line| {
            let line = line.trim();

            if line.len() == 0 || line.chars().next().unwrap() == '#' {
                None
            } else {
                parse_cedict_entry(line).or_else(|| {
                    eprintln!("Failed to parse Cedict line: {}", line);
                    None
                })
            }
        })
        .collect();

    let results = entries.len();
    println!("Number of entries: {}", results);
}

#[derive(Debug)]
struct CedictEntry<'a> {
    pub simplified: &'a str,
    pub traditional: &'a str,
    pub pinyin: &'a str,
    pub translations: Vec<&'a str>,
}

impl<'a> CedictEntry<'a> {
    fn new(
        simplified: &'a str,
        traditional: &'a str,
        pinyin: &'a str,
        translations: Vec<&'a str>,
    ) -> Self {
        Self {
            simplified,
            traditional,
            pinyin,
            translations,
        }
    }
}

fn parse_cedict_entry(line: &str) -> Option<CedictEntry> {
    let mut fields = line.split('/').filter(|l| l.len() > 0);
    let mut entry = fields.next()?.split(" [");
    let mut simp_trad = entry.next()?.split_whitespace();

    let simplified = simp_trad.next()?;
    let traditional = simp_trad.next()?;
    let pinyin = entry.next()?.trim_end().trim_end_matches(']');
    let translations = fields.collect();

    Some(CedictEntry::new(
        simplified,
        traditional,
        pinyin,
        translations,
    ))
}
