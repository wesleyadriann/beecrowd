use std::io;

fn main() {
    let mut input = String::with_capacity(12);
    let stdin = io::stdin();

    stdin.read_line(&mut input).unwrap();

    let n: u32 = input.trim().parse().unwrap();

    for i in 0..n {
        let suffix = if i == (n - 1) { "!\n" } else { " " };
        print!("Ho{}", suffix);
    }
}
