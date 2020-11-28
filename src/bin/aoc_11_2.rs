use char_combinator::CharCombinator;

// const CHARSET_FULL: &[char; 26] = &[
//     'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
//     't', 'u', 'v', 'w', 'x', 'y', 'z',
// ];

const CHARSET_WITHOUT_IOL: &[char; 23] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z',
];

fn get_next_password(s: &str) -> String {
    let charset = CHARSET_WITHOUT_IOL;

    let start: usize = s
        .clone()
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            charset
                .iter()
                .position(|c2| c == *c2)
                .map(|v| v + 1)
                .unwrap()
                * charset.len().pow(i as u32)
        })
        .sum();

    CharCombinator::new_from(start as u128, charset)
        .filter(|s| {
            let chars = s.chars().collect::<Vec<_>>();
            let len = chars.len();

            let mut i = 0;
            let mut pair_count = 0;
            loop {
                if i + 2 > len {
                    break;
                }
                if chars[i] == chars[i + 1] {
                    pair_count += 1;
                    i += 1;
                }
                i += 1;
            }
            pair_count >= 2
        })
        .filter(|s| {
            let chars = s.chars().collect::<Vec<_>>();
            let len = chars.len();
            let mut has_ascending_chars = false;
            for i in 0..len {
                if i + 2 < len {
                    let a = chars[i] as u8;
                    let b = chars[i + 1] as u8;
                    let c = chars[i + 2] as u8;
                    if a == b - 1 && a == c - 2 {
                        has_ascending_chars = true;
                    }
                } else {
                    break;
                }
            }

            has_ascending_chars
        })
        .next()
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let content = std::fs::read_to_string("data/aoc_11.txt")?;

	let pw = get_next_password(&content);

	println!("{}", pw);

	let pw = get_next_password(&pw);

	println!("{}", pw);

    Ok(())
}
