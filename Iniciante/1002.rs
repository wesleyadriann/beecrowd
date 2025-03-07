use std::io;

fn main() {
    const PI: f64 = 3.14159;

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let radius = input.trim().parse::<f64>().unwrap();

    println!("A={:.4}", (radius * radius) * PI);
}
