use std::io;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut input = String::with_capacity(18);

    let mut bet: HashSet<u32> = HashSet::with_capacity(6);
    let mut result = 0;

    stdin.read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();

    while let Some(value) = values.next().and_then(|x| x.parse::<u32>().ok()) {
        bet.insert(value);
    }

    input.clear();
    stdin.read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();

    while let Some(value) = values.next().and_then(|x| x.parse::<u32>().ok()) {
        if bet.contains(&value) {
            result += 1;
        }
    }

    match result {
        3 => println!("terno"),
        4 => println!("quadra"),
        5 => println!("quina"),
        6 => println!("sena"),
        _ => println!("azar"),
    }
}
