use std::io::{self, BufRead, BufReader};

fn main() {
    let mut reader = BufReader::new(io::stdin());

    let mut input = String::with_capacity(8);

    loop {
        input.clear();
        reader.read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            break;
        }

        let n = input.trim().parse::<u16>().unwrap();

        let mut botas = vec![String::with_capacity(8); 61];
        let mut result = 0;

        for _ in 0..n {
            input.clear();

            reader.read_line(&mut input).unwrap();

            let mut values = input.trim().split_whitespace();

            let m: usize = values.next().and_then(|x| x.parse().ok()).unwrap();
            let l = values.next().and_then(|x| Some(x.to_uppercase())).unwrap();

            let current = &mut botas[m - 30];

            if *current == "" {
                current.push_str(&l);
                continue;
            }

            if (l == "D" && current.contains("E")) || (l == "E" && current.contains("D")) {
                current.pop();
                result += 1;
            } else {
                current.push_str(&l);
            }
        }

        println!("{}", result);
    }
}
