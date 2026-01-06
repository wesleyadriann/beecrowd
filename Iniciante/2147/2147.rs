use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();
    let _ = lines.next();

    while let Some(line) = lines.next() {
        let line = line.unwrap();

        let size = line.len();
        println!("{:.2}", size as f64 / 100.0);
    }
}
