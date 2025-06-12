use std::io::{self, BufRead, BufReader};

fn main() {
    let mut reader = BufReader::new(io::stdin());

    let mut input = String::with_capacity(12);

    reader.read_line(&mut input).unwrap();

    let n = input.trim().parse::<u32>().unwrap();

    for _ in 0..n {
        input.clear();

        reader.read_line(&mut input).unwrap();

        let num = input.trim().parse::<u32>().unwrap();

        if is_prime(num) {
            println!("{} eh primo", num);
        } else {
            println!("{} nao eh primo", num);
        }
    }
}

fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }

    let limit = (num as f64).sqrt() as u32;

    for i in 2..=limit {
        if num % i == 0 {
            return false;
        }
    }

    true
}
