use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let byte = input.trim().as_bytes();

    println!("{}", byte[0] - 64);
}
