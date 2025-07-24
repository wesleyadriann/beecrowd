use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut input = String::with_capacity(6);

    loop {
        input.clear();
        reader.read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            break;
        }

        let n: usize = input.trim().parse().unwrap();

        let mut values: Vec<String> = Vec::with_capacity(n);

        for _ in 0..n {
            input.clear();
            reader.read_line(&mut input).unwrap();

            values.push(input.trim().to_string());
        }
        values.sort();
        for value in values.iter() {
            println!("{}", value);
        }
    }
}
