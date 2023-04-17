use unic::ucd::GeneralCategory as Category;

// this is a highgrain Mask that works for unicode data!
fn high_grain_unicode_mask(c: char) -> char {
    match c {
        '0'..='9' => '9',
        'a'..='z' => 'a',
        'A'..='Z' => 'A',
        c if c.is_ascii_punctuation() && (c == '"' || c == '-' || c == '.' || c == ',') => c,
        c if c.is_whitespace() => ' ',
        _ => {
            let cat = Category::of(c);

            match cat {
                Category::UppercaseLetter => 'A',
                Category::LowercaseLetter => 'a',
                Category::TitlecaseLetter => 'A',
                Category::OtherLetter => 'a',
                Category::ModifierLetter => 'a',
                Category::DecimalNumber => '9',
                Category::LetterNumber => '9',
                Category::OtherNumber => '9',
                Category::SpaceSeparator => ' ',
                Category::LineSeparator => ' ',
                Category::ParagraphSeparator => ' ',
                _ => '_',
            }
        }
    }
}

fn high_grain_mask(value: &str) -> String {
    value
        .chars()
        .map(|c| match c {
            'a'..='z' => 'a',
            'A'..='Z' => 'A',
            '0'..='9' => '9',
            _ => c,
        })
        .collect()
}

fn low_grain_mask(value: &str) -> String {
    let high_grain = high_grain_mask(value);
    let mut output = String::new();
    let mut last_char = None;

    for c in high_grain.chars() {
        if last_char != Some(c) {
            output.push(c);
            last_char = Some(c);
        }
    }
    if output.is_empty() {
        "_".to_string()
    } else {
        output
    }
}

pub(crate) fn mask_value(value: &str, grain: &str) -> String {
    match grain {
        "H" => high_grain_mask(value),
        "L" => low_grain_mask(value),
        "LU" => low_grain_mask(
            &value
                .chars()
                .map(|c| high_grain_unicode_mask(c))
                .collect::<String>(),
        ),
        _u => value.chars().map(|c| high_grain_unicode_mask(c)).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_grain_unicode_mask() {
        assert_eq!(high_grain_unicode_mask('0'), '9');
        assert_eq!(high_grain_unicode_mask('9'), '9');
        assert_eq!(high_grain_unicode_mask('a'), 'a');
        assert_eq!(high_grain_unicode_mask('z'), 'a');
        assert_eq!(high_grain_unicode_mask('A'), 'A');
        assert_eq!(high_grain_unicode_mask('Z'), 'A');
        assert_eq!(high_grain_unicode_mask('.'), '.');
        assert_eq!(high_grain_unicode_mask(','), ',');
        assert_eq!(high_grain_unicode_mask('-'), '-');
        assert_eq!(high_grain_unicode_mask('"'), '"');
        assert_eq!(high_grain_unicode_mask(' '), ' ');
        assert_eq!(high_grain_unicode_mask('➡'), '_');
    }

    #[test]
    fn test_mask_value_high_grain() {
        assert_eq!(mask_value("password123", "H"), "aaaaaaaa999");
        assert_eq!(mask_value("Name", "H"), "Aaaa");
        assert_eq!(mask_value("EMAIL@example.com", "H"), "AAAAA@aaaaaaa.aaa");
    }

    #[test]
    fn test_mask_value_low_grain() {
        assert_eq!(mask_value("password123", "L"), "a9");
        assert_eq!(mask_value("Name", "L"), "Aa");
        assert_eq!(mask_value("EMAIL@example.com", "L"), "A@a.a");
    }

    #[test]
    fn test_mask_value_low_grain_unicode() {
        assert_eq!(mask_value("password123", "LU"), "a9");
        assert_eq!(mask_value("Name", "LU"), "Aa");
        assert_eq!(mask_value("EMAIL@example.com", "LU"), "A_a.a");
    }

    #[test]
    fn test_mask_value_high_grain_unicode() {
        assert_eq!(mask_value("password123", "HU"), "aaaaaaaa999");
        assert_eq!(mask_value("Name", "HU"), "Aaaa");
        assert_eq!(mask_value("EMAIL@example.com", "HU"), "AAAAA_aaaaaaa.aaa");
    }
}
