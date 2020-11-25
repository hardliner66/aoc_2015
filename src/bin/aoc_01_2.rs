fn with_loop(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let values = content
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
		})
        .enumerate()
        .collect::<Vec<_>>();

    let mut tmp = 0;
    for (i, v) in values {
        tmp += v;
        if tmp == -1 {
            println!("{}", i + 1);
            break;
        }
    }

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

    let pos = values.iter().position(|&c| c == -1).map(|pos| pos + 1);

    println!("{:?}", pos.unwrap());

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("data/aoc_01.txt")?;
    with_loop(&content.trim())?;
    with_iterator(&content.trim())?;
    Ok(())
}