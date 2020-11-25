// >^v<

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let content = std::fs::read_to_string("data/aoc_03.txt")?;

	let mut pos = (0, 0);

	let mut positions = vec![pos];

	for c in content.chars() {
		match c {
			'^' => {
				pos.1 += 1;
			}
			'v' => {
				pos.1 -= 1;
			}
			'<' => {
				pos.0 -= 1;
			}
			'>' => {
				pos.0 += 1;
			}
			_ => (),
		}
		if !positions.contains(&pos) {
			positions.push(pos);
		}
	}

	println!("{}", positions.len());

    Ok(())
}
