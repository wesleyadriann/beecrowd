use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();

    let reader = BufReader::new(stdin.lock());

    let mut result = Vec::with_capacity(20);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            break;
        }
        result.push(line);
    }

    for (index, item) in result.iter().rev().enumerate() {
        println!("N[{}] = {}", index, item);
    }
}
