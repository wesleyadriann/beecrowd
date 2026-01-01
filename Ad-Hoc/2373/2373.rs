use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();

    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();
    let _ = lines.next();

    let mut result = 0;

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut values = line.split_whitespace();

        let l = values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();
        let c = values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();

        if l > c {
            result += c;
        }
    }

    println!("{}", result);
}
