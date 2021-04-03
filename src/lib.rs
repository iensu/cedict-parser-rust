#[derive(Debug, PartialEq)]
pub struct CedictEntry<'a> {
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

pub fn parse_cedict(contents: &str) -> Vec<CedictEntry> {
    contents
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
        .collect()
}

fn parse_cedict_entry(line: &str) -> Option<CedictEntry> {
    let mut fields = line.split('/').filter(|l| l.len() > 0);
    let mut entry = fields.next()?.split(" [");
    let mut simp_trad = entry.next()?.split_whitespace();

    let traditional = simp_trad.next()?;
    let simplified = simp_trad.next()?;
    let pinyin = entry.next()?.trim_end().trim_end_matches(']');
    let translations = fields.collect();

    Some(CedictEntry::new(
        simplified,
        traditional,
        pinyin,
        translations,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excludes_empty_lines() {
        let contents = "


";
        assert_eq!(parse_cedict(contents), vec![]);
    }

    #[test]
    fn excludes_comment_lines() {
        let contents = "
# this is a comment

# this is another comment
";
        assert_eq!(parse_cedict(contents), vec![]);
    }

    #[test]
    fn parses_a_cedict_entry() {
        let contents = "
一個蘿蔔一個坑 一个萝卜一个坑 [yi1 ge4 luo2 bo5 yi1 ge4 keng1] /lit. every turnip to its hole (idiom)/fig. each person has his own position/each to his own/horses for courses/every kettle has its lid/
";
        let entry = CedictEntry::new(
            "一个萝卜一个坑",
            "一個蘿蔔一個坑",
            "yi1 ge4 luo2 bo5 yi1 ge4 keng1",
            vec![
                "lit. every turnip to its hole (idiom)",
                "fig. each person has his own position",
                "each to his own",
                "horses for courses",
                "every kettle has its lid",
            ],
        );
        assert_eq!(parse_cedict(contents), vec![entry])
    }
}
