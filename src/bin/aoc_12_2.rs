use serde_json::{from_str, Value};

fn is_red(val: &Value) -> bool {
    if val.is_string() {
        val.as_str().iter().any(|s| *s == "red")
    } else {
        false
    }
}

fn calculate_sum(val: &Value) -> Option<i128> {
    if val.is_i64() {
        val.as_i64().map(|v| v as i128)
    } else if val.is_u64() {
        val.as_u64().map(|v| v as i128)
    } else if val.is_array() {
        val.as_array()
            .map(|v| v.iter().flat_map(calculate_sum).sum())
    } else if val.is_object() {
        val.as_object()
            .filter(|m| !m.values().any(is_red))
            .map(|v| v.values().flat_map(calculate_sum).sum())
    } else {
        Some(0)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("data/aoc_12.txt")?;

    let value: Value = from_str(&content)?;

    let sum = calculate_sum(&value);
    println!("{:?}", sum);

    Ok(())
}
