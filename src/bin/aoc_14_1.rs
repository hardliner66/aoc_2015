fn main() -> Result<(), Box<dyn std::error::Error>> {
	let content = std::fs::read_to_string("data/aoc_14.txt")?;

	let contest_duration = 2503;

    let mut parts = content
        .lines()
        .map(|line| {
            line.replace(" can fly", "")
                .replace("km/s for ", "")
                .replace("seconds, but then must rest for ", "")
                .replace(" seconds.", "")
                .split(" ")
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .flat_map(|v| {
            if let [name, speed, duration, rest] = v.as_slice() {
                Some((
                    name.clone(),
                    speed.parse::<i32>().unwrap(),
                    duration.parse::<i32>().unwrap(),
                    rest.parse::<i32>().unwrap(),
                ))
            } else {
                None
            }
		})
		.map(|(name, speed, duration, rest)| {
			let mut km = 0;
			let mut is_resting = false;
			let mut switch_duration = duration;
			for _ in 0..contest_duration {
				if !is_resting {
					km += speed
				}
				switch_duration -= 1;
				if switch_duration == 0 {
					is_resting = !is_resting;
					if is_resting {
						switch_duration = rest;
					} else {
						switch_duration = duration;
					}
				}
			}
			(name, km)
		})
		.collect::<Vec<_>>();

	parts.sort_by(|(_, km1), (_, km2)| km2.cmp(&km1));

	println!("{:?}", parts.first().unwrap());

    Ok(())
}