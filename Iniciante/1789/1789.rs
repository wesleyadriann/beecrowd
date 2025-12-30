use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();

    while let Some(_) = lines.next() {
        let next = lines.next().unwrap().unwrap();

        let mut values = next.split_whitespace();

        let mut result = 1;

        while let Some(value) = values.next().and_then(|x| x.parse::<u32>().ok()) {
            if value < 10 {
                continue;
            }

            if value < 20 && result == 2 {
                continue;
            }

            let temp = match value {
                10..=19 => 2,
                _ => 3,
            };

            if temp > result {
                result = temp;
            }

            if temp == 3 {
                break;
            }
        }

        println!("{}", result);

    }

}

