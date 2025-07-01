use std::io;

fn main() {
    let mut input = String::with_capacity(8);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut n = input.trim().parse::<i64>().unwrap();

    let mut result = if n < 1 { false } else { true };
    while n > 1 {
        if n % 2 != 0 {
            result = false;
            break;
        }
        n /= 2;
    }

    println!("{}", result);
}
