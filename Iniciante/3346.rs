use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::with_capacity(16);

    stdin.read_line(&mut input).expect("Failed to read line");

    let mut f = input.split_whitespace();

    let f1 = f.next().and_then(|x| x.parse::<f64>().ok()).unwrap();
    let f2 = f.next().and_then(|x| x.parse::<f64>().ok()).unwrap();

    let result = ((1.0 + f1 / 100.0) * (1.0 + f2 / 100.0) - 1.0) * 100.0;

    println!("{:.6}", result);
}
