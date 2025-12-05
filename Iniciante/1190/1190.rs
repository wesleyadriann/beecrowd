use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();

    let op = lines.next().unwrap().unwrap();

    let mut result = 0.0;

    let mut skip = 11;
    let mut take = 1;

    let mut lines = lines.skip(12);
    for i in 1..11 {
        let values = lines.by_ref().skip(skip).take(take);
        for value in values {
            let value = value.unwrap().parse::<f64>().unwrap();
            result += value;
        }
        if i == 5 {
            continue;
        }

        if i < 5 {
            take += 1;
            skip -= 1;
        } else {
            take -= 1;
            skip += 1;
        }
    }

    if op == "M" {
        result /= 30.0;
    }

    println!("{:.1}", result);
}
