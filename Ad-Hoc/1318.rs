use std::io::{self, BufRead, BufReader};

fn main() {
    let mut reader = BufReader::new(io::stdin());

    let mut input = String::with_capacity(20000);

    loop {
        input.clear();

        reader.read_line(&mut input).unwrap();

        if input.trim() == "0 0" {
            break;
        }

        let mut values = input.trim().split_whitespace();

        let n: usize = values.next().and_then(|x| x.parse().ok()).unwrap();

        let mut ingressos = vec![0; n];

        input.clear();

        reader.read_line(&mut input).unwrap();
        let mut t = input.split_whitespace();

        while let Some(ti) = t.next().and_then(|x| x.parse::<usize>().ok()) {
            ingressos[ti - 1] = ingressos[ti - 1] + 1;
        }

        let mut result = 0;

        for value in ingressos {
            if value > 1 {
                result += 1;
            }
        }

        println!("{}", result);
    }
}
