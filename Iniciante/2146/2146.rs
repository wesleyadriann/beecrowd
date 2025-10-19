use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();

    let reader = BufReader::new(stdin.lock());

    for line in reader.lines() {
        let line = line.unwrap();

        let value = line.trim().parse::<u32>().unwrap();

        println!("{}", value - 1);
    }
}
