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

        let mut ingressos: Vec<u32> = Vec::with_capacity(n);
        let mut duplicates: Vec<u32> = Vec::with_capacity(n);

        input.clear();

        reader.read_line(&mut input).unwrap();
        let mut t = input.split_whitespace();

        while let Some(ti) = t.next().and_then(|x| x.parse::<u32>().ok()) {
            if ingressos.contains(&ti) && !duplicates.contains(&ti) {
                duplicates.push(ti);
                continue;
            }
            ingressos.push(ti);
        }

        println!("{}", duplicates.len());
    }
}
