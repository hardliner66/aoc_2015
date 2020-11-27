enum Instruction {
	TurnOn,
	TurnOff,
	Toggle,
}

fn parse_line(line: &str) -> (Instruction, String) {
	if line.starts_with("turn on") {
		(Instruction::TurnOn, line.replace("turn on", "").trim().to_owned())
	} else if line.starts_with("turn off") {
		(Instruction::TurnOff, line.replace("turn off", "").trim().to_owned())
	} else {
		(Instruction::Toggle, line.replace("toggle", "").trim().to_owned())
	}
}

fn parse_position(text: &str) -> (i32, i32) {
	let splitted: Vec<&str> = text.split(",").collect();
	let first = splitted.first().unwrap();
	let second = splitted.last().unwrap();
	(first.parse().unwrap(), second.parse().unwrap())
}

fn parse_positions(text: &str) -> ((i32, i32), (i32, i32)) {
	let splitted: Vec<&str> = text.split("through").collect();
	let first = splitted.first().unwrap();
	let second = splitted.last().unwrap();
	(parse_position(first.trim()), parse_position(second.trim()))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let content = std::fs::read_to_string("data/aoc_06.txt")?;

	let instructions = content.lines()
		.map(parse_line)
		.map(|(i, s)| (i, parse_positions(&s)));

	let mut lights = vec![vec![0; 1000]; 1000];

	for (instruction, ((x1, y1), (x2, y2))) in instructions {
		for x in x1..=x2 {
			for y in y1..=y2 {
				let x = x as usize;
				let y = y as usize;
				match instruction {
					Instruction::TurnOn => {
						lights[x][y] += 1;
					}
					Instruction::TurnOff => {
						lights[x][y] -= 1;
						if lights[x][y] < 0 {
							lights[x][y] = 0;
						}
					}
					Instruction::Toggle => {
						lights[x][y] += 2;
					}
				}
			}
		}
	}

	let count = lights.iter().fold(0, |old, current| old + current.iter().sum::<i32>());
	println!("Count: {}", count);

	Ok(())
}