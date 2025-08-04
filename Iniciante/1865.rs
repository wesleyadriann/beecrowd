use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();

    let reader = BufReader::new(stdin.lock());

    for line in reader.lines().skip(1) {
        let line = line.unwrap();

        if &line[0..4] == "Thor" {
            println!("Y");
        } else {
            println!("N");
        }
    }
}
