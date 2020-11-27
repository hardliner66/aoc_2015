fn with_loop(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let values = content
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
		})
        .enumerate();

    let mut sum = 0;
    let mut pos = -1;
    for (i, v) in values {
        sum += v;
        if pos == -1 && sum == -1 {
            pos = i as i32 + 1;
        }
    }

    println!("Sum: {}", sum);
    println!("Pos: {}", pos);

    Ok(())
}

fn with_iterator(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let values = content
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .fold(vec![], |mut v, c| {
            let sum = if let Some(value) = v.last() {
                *value
            } else {
                0
            };

            v.push(sum + c);
            v
        });

    let sum = values.last().unwrap();
    let pos = values.iter().position(|&c| c == -1).map(|pos| pos + 1).unwrap();

    println!("Sum: {}", sum);
    println!("Pos: {}", pos);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("data/aoc_01.txt")?;
    with_loop(&content.trim())?;
    with_iterator(&content.trim())?;
    Ok(())
}