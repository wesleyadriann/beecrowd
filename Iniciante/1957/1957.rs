use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();

    let n = input.trim().parse::<u32>().unwrap();

    println!("{:X}", n);
}
