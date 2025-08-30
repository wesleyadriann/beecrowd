use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();

    let op = lines.next().unwrap().unwrap();

    let mut result = 0.0;

    let mut lines = lines.skip(78);
    let mut skip = 11;
    for _ in 0..5 {
        let mut values = lines.by_ref().skip(skip).take(13 - skip);

        while let Some(value) = values.next() {
            result += value.unwrap().trim().parse::<f64>().unwrap();
        }

        skip -= 2;
    }

    if op == "M" {
        result /= 30.0;
    }
    println!("{:.1}", result);
}
