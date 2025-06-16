use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut x: f64 = input.trim().parse().unwrap();

    for i in 0..100 {
        println!("N[{}] = {:.4}", i, x);
        x /= 2.0;
    }
}
