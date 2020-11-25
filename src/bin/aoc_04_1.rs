const INPUT: &str = "bgvyzdsv";

fn main() -> Result<(), Box<dyn std::error::Error>> {
	for i in 0..9999999 {
		let digest = md5::compute(&format!("{}{}", INPUT, i));
		let s = format!("{:x}", digest);
		if s.starts_with("00000") {
			println!("{}", i);
			break;
		}
	}

    Ok(())
}
