fn main() -> Result<(), Box<dyn std::error::Error>> {
	let value: i32 = std::fs::read_to_string("data/aoc_01.txt")?
		.trim()
		.chars()
		.map(|c| match c {
			'(' => 1,
			')' => -1,
			_ => 0,
		}).sum();
	println!("{}", value);
	Ok(())
}