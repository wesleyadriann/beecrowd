use std::io::{self, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(io::stdin());
    let mut lines = reader.lines();

    let line = lines.next().unwrap().unwrap();
    let mut values_a = line.split_whitespace();

    let line = lines.next().unwrap().unwrap();
    let mut values_b = line.split_whitespace();

    let mut result: i32 = 0;

    while let Some(value_a) = values_a.next().and_then(|x| x.parse::<i32>().ok()) {
        let value_b = values_b.next().and_then(|x| x.parse::<i32>().ok()).unwrap();
        if value_b > value_a {
            result += value_b - value_a;
        }
    }

    println!("{}", result);
}
