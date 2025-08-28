use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();

    let op = lines.next().unwrap().unwrap();

    let mut result = 0.0;

    let mut lines = lines.skip(12);
    let mut skip = 11;
    for _ in 0..11 {
        let mut values = lines.by_ref().skip(skip).take(12 - skip);

        while let Some(value) = values.next().and_then(|x| Some(x.unwrap()))  {
            result += value.trim().parse::<f64>().unwrap();
        }

        skip -= 1;
    }

    if op == "M" {
        result /= 66.0;
    }

    println!("{:.1}", result);
}

