fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("data/aoc_15.txt")?;

    let parts = content
        .lines()
        .flat_map(|line| {
            let inner = line.split(":").map(str::to_owned).collect::<Vec<String>>();
            let name = inner.first().unwrap();
            let props = inner
                .last()
                .unwrap()
                .trim()
                .replace("capacity ", "")
                .replace("durability ", "")
                .replace("flavor ", "")
                .replace("texture ", "")
                .replace("calories ", "")
                .split(",")
                .map(str::trim)
                .map(str::to_owned)
                .collect::<Vec<String>>();
            if let [capacity, durability, flavor, texture, calories] = props.as_slice() {
                Some((
                    name.clone(),
                    capacity.parse::<i32>().unwrap(),
                    durability.parse::<i32>().unwrap(),
                    flavor.parse::<i32>().unwrap(),
                    texture.parse::<i32>().unwrap(),
                    calories.parse::<i32>().unwrap(),
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut result = Vec::new();
    for a in 0..=100 {
        for b in 0..=100 {
            for c in 0..=100 {
                for d in 0..=100 {
                    if a + b + c + d == 100 {
                        result.push((
                            (a, &parts[0]),
                            (b, &parts[1]),
                            (c, &parts[2]),
                            (d, &parts[3]),
                        ));
                    }
                }
            }
        }
    }

    let mut result = result.iter().map(
        |(
            (a, (_, capacity1, durability1, flavor1, texture1, calories1)),
            (b, (_, capacity2, durability2, flavor2, texture2, calories2)),
            (c, (_, capacity3, durability3, flavor3, texture3, calories3)),
            (d, (_, capacity4, durability4, flavor4, texture4, calories4)),
        )| {
			let mut capacity =   (a * capacity1)   + (b * capacity2)   + (c * capacity3)   + (d * capacity4);
			let mut durability = (a * durability1) + (b * durability2) + (c * durability3) + (d * durability4);
			let mut flavor =     (a * flavor1)     + (b * flavor2)     + (c * flavor3)     + (d * flavor4);
			let mut texture =    (a * texture1)    + (b * texture2)    + (c * texture3)    + (d * texture4);
			let calories = (a * calories1) + (b * calories2) + (c * calories3) + (d * calories4);
			if capacity < 0 {
				capacity = 0;
			}
			if durability < 0 {
				durability = 0;
			}
			if flavor < 0 {
				flavor = 0;
			}
			if texture < 0 {
				texture = 0;
			}
			(calories, capacity * durability * flavor * texture)
		},
	).filter(|(calories, _)| *calories == 500)
	.map(|(_, score)| score)
	.collect::<Vec<_>>();

	result.sort();

	println!("{}", result.last().unwrap());

    Ok(())
}
