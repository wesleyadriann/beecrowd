use std::io::{self, BufRead, BufReader};

fn main() {
    let mut reader = BufReader::new(io::stdin());

    let mut input = String::with_capacity(6);

    loop {
        input.clear();

        reader.read_line(&mut input).unwrap();

        if input.trim() == "0" {
            break;
        }

        let n: u32 = input.trim().parse().unwrap();

        let mut a = 0;
        let mut b = 0;

        for _ in 0..n {
            input.clear();

            reader.read_line(&mut input).unwrap();

            let mut values = input.split_whitespace();

            let n1: u32 = values.next().and_then(|x| x.parse().ok()).unwrap();

            let n2: u32 = values.next().and_then(|x| x.parse().ok()).unwrap();

            if n1 == n2 {
                continue;
            } else if n1 > n2 {
                a += 1;
            } else {
                b += 1;
            }
        }

        println!("{} {}", a, b);
    }
}
