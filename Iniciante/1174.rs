use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut i = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let n: f64 = line.trim().parse().unwrap();

        if n <= 10.0 {
            println!("A[{}] = {:.1}", i, n);
        }
        i += 1;
    }
}
