use std::io;

fn main() {
    let mut input = String::with_capacity(12);

    io::stdin().read_line(&mut input).unwrap();

    let mut values = input.trim().split_whitespace();

    let a = values.next()
        .and_then(|x| x.parse::<f64>().ok())
        .unwrap();

    let b = values.next()
        .and_then(|x| x.parse::<f64>().ok())
        .unwrap();

    let result = (b / a - 1.0) * 100.0;

    println!("{:.2}%", result);
}
