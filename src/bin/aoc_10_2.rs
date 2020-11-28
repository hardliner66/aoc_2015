fn look_and_say(s: &str) -> String {
    let chars = s.chars().collect::<Vec<_>>();
	let mut i = 0;

    let mut result = Vec::new();
    let len = chars.len();
    loop {
        let c = chars[i];
        let mut count = 1;
        i += 1;
        for n in i..len {
            i = n;
            if c != chars[n] {
                break;
            }
            count += 1;
        }
        result.push(format!("{}{}", count, c));
        if i >= len {
            break;
        }
    }

    result.join("")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut content = std::fs::read_to_string("data/aoc_10.txt")?;

	for _ in 0..50 {
		content = look_and_say(&content);
	}

	println!("{}", content.len());

    Ok(())
}
