use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();

    let op = lines.next().unwrap().unwrap();

    let mut result = 0.0;

    let mut skip = 1;
    let mut take = 10;
    for _ in 0..5 {
        let values = lines.by_ref().skip(skip).take(take);
        for value in values {
            result += value.unwrap().trim().parse::<f64>().unwrap();
        }
        skip += 2;
        take -= 2;
    }

    if op == "M" {
        result /= 30.0;
    }

    println!("{:.1}", result);
}
