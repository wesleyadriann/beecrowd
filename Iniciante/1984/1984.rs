use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let result: String = input.trim()
        .chars()
        .rev()
        .collect();

    println!("{}", result);
}
