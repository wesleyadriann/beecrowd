use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines() {
        let line = line.unwrap();

        if line.trim() == "0" {
            println!("vai ter copa!");
        } else {
            println!("vai ter duas!");
        }
    }
}
