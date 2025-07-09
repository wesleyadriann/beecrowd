use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();

    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();

    let op = lines.next().unwrap().unwrap();

    let mut result = 0.0;

    let mut skip: usize = 0;
    for _ in 0..12 {
        skip += 1;
        let values = lines.by_ref().skip(skip).take(12 - skip);
        for value in values {
            result += value.unwrap().trim().parse::<f64>().unwrap();
        }
    }

    if op == "M" {
        result /= 66.0;
    }
    println!("{:.1}", result);
}
