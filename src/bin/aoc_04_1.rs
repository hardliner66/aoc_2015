fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("data/aoc_04.txt")?;
    let mut i = 0;
    loop {
        let digest = md5::compute(&format!("{}{}", content, i));
        let s = format!("{:x}", digest);
        if s.starts_with("00000") {
            println!("{}", i);
            break;
        }
        i += 1;
    }

    Ok(())
}
