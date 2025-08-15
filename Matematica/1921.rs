use std::io;

fn main() {
    let mut input = String::with_capacity(6);

    io::stdin().read_line(&mut input).unwrap();

    let n: u64 = input.trim().parse().unwrap();

    println!("{}", (n * (n - 3)) / 2);
}
