use std::io;

fn main() {
    let mut input = String::with_capacity(10);

    io::stdin().read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let h = values.next()
        .and_then(|x| x.parse::<f64>().ok())
        .unwrap();
    let p = values.next()
        .and_then(|x| x.parse::<f64>().ok())
        .unwrap();

    println!("{:.2}", h / p);
}
