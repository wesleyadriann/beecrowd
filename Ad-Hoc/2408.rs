use std::{io, cmp};

fn main() {
    let mut input = String::with_capacity(12);

    io::stdin().read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let a = values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();
    let b = values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();
    let c = values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();

    let result = a + b + c - cmp::max(a, cmp::max(b, c)) - cmp::min(a, cmp::min(b, c));

    println!("{}", result);
}
