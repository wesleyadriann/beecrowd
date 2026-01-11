use std::io;

fn main() {
    let mut input = String::with_capacity(4);

    io::stdin().read_line(&mut input).unwrap();

    let value: f64 = input.trim()
        .parse()
        .unwrap();

    let sqrt_five = (5.0_f64).sqrt();

    let result = (
        ((1.0 + sqrt_five) / 2.0).powf(value) -
        ((1.0 - sqrt_five) / 2.0).powf(value)
    ) / sqrt_five;

    println!("{:.1}", result);
}
