fn is_nice(line: &str) -> bool {
    let chars = line.chars().collect::<Vec<_>>();

    let mut pairs = Vec::new();

    for i in 0..chars.len()-1 {
        pairs.push(format!("{}{}", chars[i], chars[i+1]));
	}

	let mut non_overlapping_pair_found = false;
	for pair in &pairs {
		let pos = pairs.iter().position(|p| p == pair).unwrap();
		for i in 0..pairs.len() {
			if i == pos || i == pos + 1 {
				continue;
			}
			if pair == &pairs[i] {
				non_overlapping_pair_found = true;
				break;
			}
		}
	}

	let mut repeating_char_with_character_between = false;
	for i in 0..chars.len()-2 {
		if chars[i] == chars[i+2] {
			repeating_char_with_character_between = true;
			break;
		}
	}

	non_overlapping_pair_found && repeating_char_with_character_between
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("data/aoc_05.txt")?;

    let mut good_lines = Vec::new();

    for line in content.lines() {
        if is_nice(line) {
            good_lines.push(line);
        }
    }

    println!("{}", good_lines.len());

    Ok(())
}
