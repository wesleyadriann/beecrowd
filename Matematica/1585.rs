use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines().skip(1) {
        let line = line.unwrap();

        let mut values = line.split_whitespace();

        let d1 = values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();
        let d2 = values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();

        println!("{} cm2", d1 * d2 / 2);
    }
}
