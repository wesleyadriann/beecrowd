use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines().skip(1) {
        let line = line.unwrap();

        let n = line.trim().parse::<u32>().unwrap();

        if n % 2 == 0 {
            println!("0");
        } else {
            println!("1");
        }
    }
}
