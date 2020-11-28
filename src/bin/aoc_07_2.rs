use std::{cmp::Ordering, collections::HashMap, str::FromStr};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Source {
    Reference(String),
    Value(u16),
}

impl FromStr for Source {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Ok(v) = s.parse::<u16>() {
            Source::Value(v)
        } else {
            Source::Reference(s.to_owned())
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Operation {
    Assign(Source, String),
    Not(Source, String),
    Or(Source, Source, String),
    And(Source, Source, String),
    LShift(Source, u16, String),
    RShift(Source, u16, String),
}

impl Ord for Operation {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Operation::Assign(_, _) => match other {
                Operation::Assign(_, _) => Ordering::Equal,
                _ => Ordering::Less,
            },
            _ => match other {
                Operation::Assign(_, _) => Ordering::Greater,
                _ => Ordering::Equal,
            },
        }
    }
}

impl PartialOrd for Operation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_unary(s: &str, splitter: &str) -> (Source, String) {
    let parts = s
        .replace(&format!("{} ", splitter), "")
        .split("->")
        .map(str::trim)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    (
        parts.first().unwrap().parse().unwrap(),
        parts.last().unwrap().to_owned(),
    )
}

fn parse_binary(s: &str, splitter: &str) -> (Source, Source, String) {
    let parts = s
        .split(&format!("{}", splitter))
        .map(str::trim)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let first = parts.first().unwrap();
    let parts = parts
        .last()
        .unwrap()
        .split("->")
        .map(str::trim)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    (
        first.parse().unwrap(),
        parts.first().unwrap().parse().unwrap(),
        parts.last().unwrap().to_owned(),
    )
}

fn parse_binary_with_value(s: &str, splitter: &str) -> (Source, u16, String) {
    let parts = s
        .split(&format!("{}", splitter))
        .map(str::trim)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let first = parts.first().unwrap();
    let parts = parts
        .last()
        .unwrap()
        .split("->")
        .map(str::trim)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    (
        first.parse().unwrap(),
        parts.first().unwrap().to_owned().parse().unwrap(),
        parts.last().unwrap().to_owned(),
    )
}

fn parse_assign(s: &str) -> (Source, String) {
    let parts = s
        .split("->")
        .map(str::trim)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let first = parts.first().unwrap();
    (
        first
            .parse()
            .map(Source::Value)
            .unwrap_or(Source::Reference(first.to_owned())),
        parts.last().unwrap().to_owned(),
    )
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("NOT") {
            let (s1, s2) = parse_unary(s, "NOT");
            Ok(Operation::Not(s1, s2))
        } else if s.contains("AND") {
            let (s1, s2, s3) = parse_binary(s, "AND");
            Ok(Operation::And(s1, s2, s3))
        } else if s.contains("OR") {
            let (s1, s2, s3) = parse_binary(s, "OR");
            Ok(Operation::Or(s1, s2, s3))
        } else if s.contains("LSHIFT") {
            let (s1, s2, s3) = parse_binary_with_value(s, "LSHIFT");
            Ok(Operation::LShift(s1, s2, s3))
        } else if s.contains("RSHIFT") {
            let (s1, s2, s3) = parse_binary_with_value(s, "RSHIFT");
            Ok(Operation::RShift(s1, s2, s3))
        } else {
            let (s1, s2) = parse_assign(s);
            Ok(Operation::Assign(s1, s2))
        }
    }
}

fn get_from_wires(wires: &HashMap<String, u16>, src: &Source) -> Option<u16> {
    match src {
        Source::Value(v) => Some(*v),
        Source::Reference(r) => wires.get(r).cloned(),
    }
}

fn execute(operators: Vec<Operation>) -> HashMap<String, u16> {
    let mut wires: HashMap<String, u16> = HashMap::new();

    loop {
        let len = operators.len();
        let operators = operators
            .iter()
            .filter(|op| {
                match &op {
                    Operation::Assign(Source::Value(v), dst) => {
                        *wires.entry(dst.clone()).or_insert(*v) = *v;
                        return false;
                    }
                    Operation::Assign(Source::Reference(src), dst) => {
                        if let Some(v) = wires.get(src).cloned() {
                            *wires.entry(dst.clone()).or_insert(v) = v;
                            return false;
                        }
                    }
                    Operation::Not(src, dst) => {
                        if let Some(v) = get_from_wires(&wires, src) {
                            *wires.entry(dst.clone()).or_insert(!v) = !v;
                            return false;
                        }
                    }
                    Operation::And(src1, src2, dst) => {
                        if let Some(v1) = get_from_wires(&wires, src1) {
                            if let Some(v2) = get_from_wires(&wires, src2) {
                                let v = v1 & v2;
                                *wires.entry(dst.clone()).or_insert(v) = v;
                                return false;
                            }
                        }
                    }
                    Operation::Or(src1, src2, dst) => {
                        if let Some(v1) = get_from_wires(&wires, src1) {
                            if let Some(v2) = get_from_wires(&wires, src2) {
                                let v = v1 | v2;
                                *wires.entry(dst.clone()).or_insert(v) = v;
                                return false;
                            }
                        }
                    }
                    Operation::LShift(src, bits, dst) => {
                        if let Some(v) = get_from_wires(&wires, src) {
                            let v = v << *bits;
                            *wires.entry(dst.clone()).or_insert(v) = v;
                            return false;
                        }
                    }
                    Operation::RShift(src, bits, dst) => {
                        if let Some(v) = get_from_wires(&wires, src) {
                            let v = v >> *bits;
                            *wires.entry(dst.clone()).or_insert(v) = v;
                            return false;
                        }
                    }
                }
                true
            })
            .collect::<Vec<_>>();

        if operators.len() == len || operators.len() == 0 {
            break;
        }
    }

    wires
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("data/aoc_07.txt")?;

    let mut operators = content
        .lines()
        .flat_map(str::parse::<Operation>)
        .collect::<Vec<_>>();
    operators.sort();

    let wires = execute(operators.clone());
    let a = wires.get("a").unwrap();

    println!("a: {}", a);

    let wires2 = execute(
        operators
            .iter()
            .map(|op| match op {
                Operation::Assign(_, dst) if dst == "b" => {
                    Operation::Assign(Source::Value(*a), dst.clone())
                }
                other => other.clone(),
            })
            .collect::<Vec<_>>(),
    );

    let a = wires2.get("a").unwrap();

    println!("a: {}", a);

    Ok(())
}
