use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("data/aoc_14.txt")?;

    let contest_duration = 2503;

    let parts = content
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
            let mut kms = vec![];
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
                kms.push(km);
            }
            (name, kms)
        })
        .collect::<Vec<_>>();

    let mut points = HashMap::new();
	for (name, _) in &parts {
		points.insert(name, 0);
	}
    for i in 0..contest_duration {
		let max = parts.iter().fold(0, |acc,(_, value)| if acc > value[i] {
			acc
		} else {
			value[i]
		});
        for (name, _) in parts.iter().filter(|(_, kms)| kms[i] == max) {
			*points.entry(name).or_insert(1) += 1;
		}
	}

	let mut points = points.iter().collect::<Vec<_>>();
	points.sort_by(|(_, points1), (_, points2)| points2.cmp(&points1));

    println!("{:?}", points.first().unwrap());

    Ok(())
}
