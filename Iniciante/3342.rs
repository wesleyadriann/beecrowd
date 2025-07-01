use std::io;

fn main() {
    let mut input = String::with_capacity(4);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u32 = input.trim().parse().unwrap();

    let mut white = n * n / 2;
    let black = n * n / 2;

    if n % 2 != 0 {
        white += 1;
    }
    println!("{} casas brancas e {} casas pretas", white, black);
}
