use std::io;

fn main() {
    let stdin = io::stdin();

    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();

    let n: u32 = input.trim().parse().unwrap();

    input.clear();
    stdin.read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let a: u32 = values.next().and_then(|x| x.parse().ok()).unwrap();
    let l: u32 = values.next().and_then(|x| x.parse().ok()).unwrap();
    let p: u32 = values.next().and_then(|x| x.parse().ok()).unwrap();

    if a < n || l < n || p < n {
        println!("N");
    } else {
        println!("S");
    }
}
