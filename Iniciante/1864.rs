use std::io;

fn main() {
    const PHARSE: &str = "LIFE IS NOT A PROBLEM TO BE SOLVED";

    let mut input = String::with_capacity(4);

    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<usize>().unwrap();

    println!("{}", &PHARSE[0..n]);
}
