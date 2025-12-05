use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();

    let op = lines.next().unwrap().unwrap();

    let mut result = 0.0;

    let mut skip = 12;
    let mut take = 1;
    for i in 0..10 {
        let values = lines.by_ref().skip(skip).take(take);
        for value in values {
            result += value.unwrap().trim().parse::<f64>().unwrap();
        }

        if i == 4 {
            skip -= 1;
        } else if i == 5 {
            take -= 1;
        } else if i < 4 {
            skip -= 1;
            take += 1;
        } else {
            skip += 1;
            take -= 1;
        }
    }

    if op == "M" {
        result /= 30.0;
    }

    println!("{:.1}", result);

}
