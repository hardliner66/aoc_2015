use hex::FromHex;

fn unescape(orig: &str) -> String {
    let s = &orig[1..orig.len() - 1];
    let chars = s.chars().collect::<Vec<_>>();

    let mut chars_unescaped = Vec::new();

    if chars.len() > 0 {
        let mut i = 0;
        loop {
            if chars[i] == '\\' && i < chars.len() - 1 {
                if chars[i + 1] == '\\' {
                    chars_unescaped.push('\\' as u8);
                    i += 2;
                } else if chars[i + 1] == '"' {
                    chars_unescaped.push('"' as u8);
                    i += 2;
                } else if chars[i + 1] == 'x' && i < chars.len() - 3 {
                    let hex_chars = [chars[i + 2] as u8, chars[i + 3] as u8];
                    let hex_string: String = String::from_utf8_lossy(&hex_chars).into();
                    let hex = <[u8; 1]>::from_hex(&hex_string).expect("Decoding failed");
                    chars_unescaped.push(hex[0]);
                    i += 4;
                } else {
                    i += 1;
                }
            } else {
                chars_unescaped.push(chars[i] as u8);
                i += 1;
            }
            if i >= chars.len() {
                break;
            }
        }
    }

    String::from_utf8_lossy(&chars_unescaped).into()
}

fn escape(orig: &str) -> String {
    format!("\"{}\"", orig.replace("\\", "\\\\").replace("\"", "\\\""))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("data/aoc_08.txt")?;

    let sum: usize = content
        .lines()
        .map(str::trim)
        .map(|line| (line.chars().count(), unescape(line).chars().count()))
        .map(|(code_length, string_length)| code_length - string_length)
        .sum();

    println!("Part 1: {}", sum);

    let sum: usize = content
        .lines()
        .map(str::trim)
        .map(|line| (line.chars().count(), escape(line).chars().count()))
        .map(|(code_length, string_length)| string_length - code_length)
        .sum();

    println!("Part 2: {}", sum);

    Ok(())
}
