const VOWELS: &[char; 5] = &['a', 'e', 'i', 'o', 'u'];

fn is_nice(line: &str) -> bool {
	for s in &["ab", "cd", "pq", "xy"] {
		if line.contains(s) {
			return false;
		}
	}
	let chars = line.chars().collect::<Vec<_>>();

	let mut vowels = 0;
	for c in &chars {
		if VOWELS.contains(c) {
			vowels += 1;
		}
	}

	let mut has_double_letter = false;
	for (i, s) in chars.iter().enumerate() {
		if i < chars.len() - 1 {
			if *s == chars[i+1] {
				has_double_letter = true;
				break;
			}
		}
	}

	vowels >= 3 && has_double_letter
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
