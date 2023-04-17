use crate::parser::tabular::LineReader;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn init_control_character_descriptions() -> HashMap<char, &'static str> {
    let mut ref_map = HashMap::new();
    ref_map.insert('\u{0000}', "NUL - Null char");
    ref_map.insert('\u{0001}', "SOH - Start of Heading");
    ref_map.insert('\u{0002}', "STX - Start of Text");
    ref_map.insert('\u{0003}', "ETX - End of Text");
    ref_map.insert('\u{0004}', "EOT - End of Transmission");
    ref_map.insert('\u{0005}', "ENQ - Enquiry");
    ref_map.insert('\u{0006}', "ACK - Acknowledgment");
    ref_map.insert('\u{0007}', "BEL - Bell");
    ref_map.insert('\u{0008}', "BS - Back Space");
    ref_map.insert('\u{0009}', "HT - Horizontal Tab");
    ref_map.insert('\u{000A}', "LF - Line Feed");
    ref_map.insert('\u{000B}', "VT - Vertical Tab");
    ref_map.insert('\u{000C}', "FF - Form Feed");
    ref_map.insert('\u{000D}', "CR - Carriage Return");
    ref_map.insert('\u{000E}', "SO - Shift Out / X-On");
    ref_map.insert('\u{000F}', "SI - Shift In / X-Off");
    ref_map.insert('\u{0010}', "DLE - Data Line Escape");
    ref_map.insert('\u{0011}', "DC1 - Device Control 1 (oft. XON)");
    ref_map.insert('\u{0012}', "DC2 - Device Control 2");
    ref_map.insert('\u{0013}', "DC3 - Device Control 3 (oft. XOFF)");
    ref_map.insert('\u{0014}', "DC4 - Device Control 4");
    ref_map.insert('\u{0015}', "NAK - Negative Acknowledgement");
    ref_map.insert('\u{0016}', "SYN - Synchronous Idle");
    ref_map.insert('\u{0017}', "ETB - End of Transmit Block");
    ref_map.insert('\u{0018}', "CAN - Cancel");
    ref_map.insert('\u{0019}', "EM - End of Medium");
    ref_map.insert('\u{001A}', "SUB - Substitute");
    ref_map.insert('\u{001B}', "ESC - Escape");
    ref_map.insert('\u{001C}', "FS - File Separator");
    ref_map.insert('\u{001D}', "GS - Group Separator");
    ref_map.insert('\u{001E}', "RS - Record Separator");
    ref_map.insert('\u{001F}', "US - Unit Separator");
    ref_map.insert(
        '\u{008A}',
        "LINE TABULATION SET * Deprecated from Unicode 3.2, 2002",
    );
    ref_map.insert('\u{0090}', "ERROR - Undefined CTRL Character.");
    ref_map.insert('\u{009A}', "LATIN CAPITAL S WITH CARON");
    ref_map.insert('\u{FDD0}', "Non-character code point");
    ref_map.insert('\u{FDD1}', "Non-character code point");
    ref_map.insert('\u{FDD2}', "Non-character code point");
    ref_map.insert('\u{FDD3}', "Non-character code point");
    ref_map.insert('\u{FDD4}', "Non-character code point");
    ref_map.insert('\u{FDD5}', "Non-character code point");
    ref_map.insert('\u{FDD6}', "Non-character code point");
    ref_map.insert('\u{FDD7}', "Non-character code point");
    ref_map.insert('\u{FDD8}', "Non-character code point");
    ref_map.insert('\u{FDD9}', "Non-character code point");
    ref_map.insert('\u{FDDA}', "Non-character code point");
    ref_map.insert('\u{FDDB}', "Non-character code point");
    ref_map.insert('\u{FDDC}', "Non-character code point");
    ref_map.insert('\u{FDDD}', "Non-character code point");
    ref_map.insert('\u{FDDE}', "Non-character code point");
    ref_map.insert('\u{FDDF}', "Non-character code point");
    ref_map.insert('\u{FDE0}', "Non-character code point");
    ref_map.insert('\u{FDE1}', "Non-character code point");
    ref_map.insert('\u{FDE2}', "Non-character code point");
    ref_map.insert('\u{FDE3}', "Non-character code point");
    ref_map.insert('\u{FDE4}', "Non-character code point");
    ref_map.insert('\u{FDE5}', "Non-character code point");
    ref_map.insert('\u{FDE6}', "Non-character code point");
    ref_map.insert('\u{FDE7}', "Non-character code point");
    ref_map.insert('\u{FDE8}', "Non-character code point");
    ref_map.insert('\u{FDE9}', "Non-character code point");
    ref_map.insert('\u{FDEA}', "Non-character code point");
    ref_map.insert('\u{FDEB}', "Non-character code point");
    ref_map.insert('\u{FDEC}', "Non-character code point");
    ref_map.insert('\u{FDED}', "Non-character code point");
    ref_map.insert('\u{FDEE}', "Non-character code point");
    ref_map.insert('\u{FDEF}', "Non-character code point");
    ref_map.insert('\u{FFFA}', "Undefined Control Character");
    ref_map.insert('\u{FFFB}', "Undefined Control Character");
    ref_map.insert('\u{FFFC}', "Undefined Control Character");
    ref_map.insert('\u{1FFFE}', "Undefined Control Character");
    ref_map.insert('\u{1FFFF}', "Undefined Control Character");
    ref_map.insert('\u{2FFFE}', "Undefined Control Character");
    ref_map.insert('\u{2FFFF}', "Undefined Control Character");
    ref_map.insert('\u{3FFFE}', "Undefined Control Character");
    ref_map.insert('\u{3FFFF}', "Undefined Control Character");
    ref_map.insert('\u{4FFFE}', "Undefined Control Character");
    ref_map.insert('\u{4FFFF}', "Undefined Control Character");
    ref_map.insert('\u{5FFFE}', "Undefined Control Character");
    ref_map.insert('\u{5FFFF}', "Undefined Control Character");
    ref_map.insert('\u{6FFFE}', "Undefined Control Character");
    ref_map.insert('\u{6FFFF}', "Undefined Control Character");
    ref_map.insert('\u{7FFFE}', "Undefined Control Character");
    ref_map.insert('\u{7FFFF}', "Undefined Control Character");
    ref_map.insert('\u{8FFFE}', "Undefined Control Character");
    ref_map.insert('\u{8FFFF}', "Undefined Control Character");
    ref_map.insert('\u{9FFFE}', "Undefined Control Character");
    ref_map.insert('\u{9FFFF}', "Undefined Control Character");
    ref_map.insert('\u{AFFFE}', "Undefined Control Character");
    ref_map.insert('\u{AFFFF}', "Undefined Control Character");
    ref_map.insert('\u{BFFFE}', "Undefined Control Character");
    ref_map.insert('\u{BFFFF}', "Undefined Control Character");
    ref_map.insert('\u{CFFFE}', "Undefined Control Character");
    ref_map.insert('\u{CFFFF}', "Undefined Control Character");
    ref_map.insert('\u{DFFFE}', "Undefined Control Character");
    ref_map.insert('\u{DFFFF}', "Undefined Control Character");
    ref_map.insert('\u{EFFFE}', "Undefined Control Character");
    ref_map.insert('\u{EFFFF}', "Undefined Control Character");
    ref_map.insert('\u{FFFFE}', "Undefined Control Character");
    ref_map.insert('\u{FFFFF}', "Undefined Control Character");
    ref_map.insert('\u{10FFFE}', "Undefined Control Character");
    ref_map.insert('\u{10FFFF}', "Undefined Control Character");

    ref_map
}

pub(crate) fn character_profiling() -> Result<(), io::Error> {
    let ascii_control_characters = init_control_character_descriptions();
    let stdin = io::stdin();
    let mut frequency_map: HashMap<char, usize> = HashMap::new();

    let file_reader: Box<dyn BufRead> = Box::new(stdin.lock());

    let mut reader = LineReader::new(file_reader);

    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        for c in line.chars() {
            let count = frequency_map.entry(c).or_insert(0);
            *count += 1;
        }
        line.clear();
    }

    println!(
        "{:<8}\t{:<8}\t{}\t{}",
        "char", "count", "description", "name"
    );
    println!("{:-<8}\t{:-<8}\t{:-<15}\t{:-<15}", "", "", "", "");

    let mut sorted_chars: Vec<(char, usize)> = frequency_map.into_iter().collect();
    sorted_chars.sort_unstable_by_key(|&(c, _)| c as u32);

    for (c, count) in sorted_chars {
        let character_name = unicode_names2::name(c).map_or_else(
            || {
                ascii_control_characters
                    .get(&c)
                    .map_or("UNKNOWN".to_string(), |desc| desc.to_string())
            },
            |name| name.to_string(),
        );
        println!(
            "{:<8}\t{:<8}\t{}\t{}",
            c.escape_unicode(),
            count,
            c.escape_debug(),
            character_name
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_control_character_descriptions() {
        let control_char_descriptions = init_control_character_descriptions();

        // Test a few control characters to ensure they have the expected descriptions
        assert_eq!(
            control_char_descriptions.get(&'\u{0000}'),
            Some(&"NUL - Null char")
        );
        assert_eq!(
            control_char_descriptions.get(&'\u{0001}'),
            Some(&"SOH - Start of Heading")
        );
        assert_eq!(
            control_char_descriptions.get(&'\u{0002}'),
            Some(&"STX - Start of Text")
        );
        assert_eq!(
            control_char_descriptions.get(&'\u{0003}'),
            Some(&"ETX - End of Text")
        );
        assert_eq!(
            control_char_descriptions.get(&'\u{0004}'),
            Some(&"EOT - End of Transmission")
        );

        // Test a few undefined control characters to ensure they are labeled as such
        assert_eq!(
            control_char_descriptions.get(&'\u{0090}'),
            Some(&"ERROR - Undefined CTRL Character.")
        );
        assert_eq!(
            control_char_descriptions.get(&'\u{FDD0}'),
            Some(&"Non-character code point")
        );
        assert_eq!(
            control_char_descriptions.get(&'\u{FDD1}'),
            Some(&"Non-character code point")
        );

        // Ensure there are no unexpected control characters in the map
        assert_eq!(control_char_descriptions.len(), 102);
    }
}
