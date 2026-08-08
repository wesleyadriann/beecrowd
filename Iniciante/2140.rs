use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines() {
        let line = line.unwrap();

        if line.trim() == "0 0" {
            break;
        }

        let mut values = line.split_whitespace();

        let n: u32 = values.next().and_then(|x| x.parse().ok()).unwrap();
        let m: u32 = values.next().and_then(|x| x.parse().ok()).unwrap();

        let mut r = m - n;

        if r > 200 {
            println!("impossible");
            continue;
        }

        let mut t: u32 = 0;

        if r / 100 >= 1 {
            t += 1;
            r = r % 100;
        }

        if r / 50 >= 1 {
            t += 1;
            r = r % 50;
        }

        if r / 20 >= 1 {
            t += 1;
            r = r % 20;
        }

        if r / 10 >= 1 {
            t += 1;
            r = r % 10;
        }

        if r / 5 >= 1 {
            t += 1;
            r = r % 5;
        }

        if r / 2 >= 1 {
            t += 1;
        }

        if t == 2 {
            println!("possible");
        } else {
            println!("impossible");
        }
    }
}
