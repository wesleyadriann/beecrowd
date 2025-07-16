use std::io::{self, BufRead, BufReader};

fn main() {
    let mut reader = BufReader::new(io::stdin());

    let mut input = String::with_capacity(64);

    reader.read_line(&mut input).unwrap();

    let n: u32 = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();

        let m: u32 = input.trim().parse().unwrap();

        let mut result = 0;

        for i in 0..m {
            input.clear();
            reader.read_line(&mut input).unwrap();

            let line = input.trim().to_string();
            let mut chars = line.chars();

            let mut j = 0;
            while let Some(c) = chars.next() {
                let pos = char_index(c) as u32;
                result += pos + i + j;
                j += 1;
            }
        }

        println!("{}", result);
    }
}

fn char_index(c: char) -> u32 {
    let ascii_value = c as u32;

    ascii_value - 65
}
