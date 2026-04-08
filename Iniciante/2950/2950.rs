use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut values = input.trim()
        .split_whitespace();

    let n: f64 = values.next().and_then(|x| x.parse().ok()).unwrap();
    let x: u32 = values.next().and_then(|x| x.parse().ok()).unwrap();
    let y: u32 = values.next().and_then(|x| x.parse().ok()).unwrap();

    let x_y = (x + y) as f64;

    println!("{:.2}", n / x_y);
}


