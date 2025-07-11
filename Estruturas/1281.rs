use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};

fn main() {
    let mut reader = BufReader::new(io::stdin());

    let mut input = String::with_capacity(32);

    reader.read_line(&mut input).unwrap();

    let n: u32 = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();

        let n = input.trim().parse::<usize>().unwrap();

        let mut products: HashMap<_, f64> = HashMap::with_capacity(n);

        for _ in 0..n {
            input.clear();

            reader.read_line(&mut input).unwrap();

            let line = input.clone();
            let mut values = line.split_whitespace();

            let name = values.next().unwrap().to_string();
            let value = values.next().and_then(|x| x.parse::<f64>().ok()).unwrap();

            products.insert(name, value);
        }

        input.clear();
        reader.read_line(&mut input).unwrap();

        let m = input.trim().parse::<u32>().unwrap();

        let mut result = 0.0;

        for _ in 0..m {
            input.clear();

            reader.read_line(&mut input).unwrap();

            let mut values = input.split_whitespace();

            let name = values.next().unwrap();
            let qtd = values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();

            let value = products.get(name).unwrap_or(&0.0);

            result += value * qtd as f64;
        }

        println!("R$ {:.2}", result);
    }
}
