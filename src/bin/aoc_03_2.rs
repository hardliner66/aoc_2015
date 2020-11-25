// >^v<

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let content = std::fs::read_to_string("data/aoc_03.txt")?;

	let mut pos1 = (0, 0);
	let mut pos2 = (0, 0);

	let mut positions = vec![pos1];

	let mut is_real_santa = true;

	for c in content.chars() {
		let mut pos = if is_real_santa {
			&mut pos1
		} else {
			&mut pos2
		};
		is_real_santa = !is_real_santa;
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
			positions.push(*pos);
		}
	}

	println!("{}", positions.len());

    Ok(())
}
