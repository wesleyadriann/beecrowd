use std::io;

fn main() {
    let stdin = io::stdin();

    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();

    input.clear();
    stdin.read_line(&mut input).unwrap();

    let values = input.trim().split_whitespace();

    let mut y: u32 = 0;
    let mut n: u32 = 0;

    for value in values {
        if value == "0" {
            y += 1;
        } else {
            n += 1;
        }
    }

    if y > n {
        println!("Y");
    } else {
        println!("N");
    }
}
