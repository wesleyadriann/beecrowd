use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let mut bigger = 0;

    while let Some(value) = values.next() {
        let value: i32 = value.parse().unwrap();
        if value > bigger {
            bigger = value;
        }
    }
    println!("{}", bigger);
}
