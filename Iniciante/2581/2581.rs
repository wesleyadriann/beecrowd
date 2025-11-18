use std::io;

fn main() {
    let mut input = String::with_capacity(8);

    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<u32>().unwrap();

    for _ in 0..n {
        println!("I am Toorg!");
    }
}
