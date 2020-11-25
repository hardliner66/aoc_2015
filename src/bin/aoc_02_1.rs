fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("data/aoc_02.txt")?;

    let values = content.lines().map(|s| s.split("x")).map(|mut s| {
        let mut v = vec![
            s.next().unwrap().parse::<i64>().unwrap(),
            s.next().unwrap().parse::<i64>().unwrap(),
            s.next().unwrap().parse::<i64>().unwrap(),
		];
		v.sort();
		v
    });

    let result: i64 = values.map(|v| {
		if let [a, b, c] = &v.as_slice() {
			// we take the first one 3 times,
			// because the smallest sides are used to calculate the extra paper
			(3 * a * b) + (2 * b * c) + (2 * c * a)
		} else {
			0
		}
	}).sum();

	println!("{}", result);

    Ok(())
}
