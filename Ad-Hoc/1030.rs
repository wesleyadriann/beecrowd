use std::io::{self, BufRead, BufReader};

fn main() {
    let mut reader = BufReader::new(io::stdin());

    let mut input = String::with_capacity(10);

    reader.read_line(&mut input).unwrap();

    let nc: u32 = input.trim().parse().unwrap();

    for i in 0..nc {
        input.clear();
        reader.read_line(&mut input).unwrap();

        let mut values = input.split_whitespace();

        let n = values.next().and_then(|s| s.parse::<u32>().ok()).unwrap();

        let k = values.next().and_then(|s| s.parse::<u32>().ok()).unwrap();

        let mut s = 0;

        for j in 0..n {
            s = (s + k) % (j + 1);
        }

        println!("Case {}: {}", i + 1, s + 1);
    }
}
