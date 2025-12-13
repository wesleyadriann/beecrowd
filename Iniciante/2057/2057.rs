use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::with_capacity(10);

    stdin.read_line(&mut input).unwrap();

    let mut t: i32 = input.split_whitespace()
        .fold(0, |acc, x| {
            let value = x.parse::<i32>().unwrap();
            acc + value
        });

    if t > 24 {
        t -= 24;
    } else if t < 0 {
        t += 24;
    }

    println!("{}", t);
}
