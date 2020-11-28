fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("data/aoc_09.txt")?;

    let v = content
        .lines()
        .map(|line| {
            line.split("to")
                .flat_map(|s| s.split("="))
                .map(str::trim)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .map(|v| {
            (
                v.first().unwrap().clone(),
                v.get(1).unwrap().clone(),
                v.last().unwrap().clone(),
            )
        })
		.collect::<Vec<_>>();

	println!("{:#?}", v);

    Ok(())
}
