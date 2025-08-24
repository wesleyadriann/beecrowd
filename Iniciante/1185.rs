use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut skip = 0;
    let mut result = 0.0;

    let mut lines = reader.lines();

    let op = lines.next().unwrap().unwrap();

    for _ in 0..11 {
        let values = lines.by_ref().take(12);

        for (i, line) in values.enumerate() {
            let value = line.unwrap().trim().parse::<f64>().unwrap();
            if i < 11 - skip {
                result += value;
            }
        }

        skip += 1;
    }

    if op == "M" {
        result /= 66.0;
    }

    println!("{:.1}", result);
}
