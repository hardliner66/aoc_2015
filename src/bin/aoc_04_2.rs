fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("data/aoc_04.txt")?;
    for i in 0..9999999 {
        let digest = md5::compute(&format!("{}{}", content, i));
        let s = format!("{:x}", digest);
        if s.starts_with("000000") {
            println!("{}", i);
            break;
        }
    }

    Ok(())
}
