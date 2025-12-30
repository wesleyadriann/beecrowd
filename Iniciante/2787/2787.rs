use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::with_capacity(6);

    stdin.read_line(&mut input).unwrap();

    let l: u32 = input.trim().parse().unwrap();

    input.clear();
    stdin.read_line(&mut input).unwrap();

    let c: u32 = input.trim().parse().unwrap();

    if (l % 2 == 0 && c % 2 == 0) || (l % 2 != 0 && c % 2 != 0) {
        println!("1");
    } else {
        println!("0");
    }
}
