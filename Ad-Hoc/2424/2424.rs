use std::io;

fn main() {
    let mut input = String::with_capacity(10);

    io::stdin().read_line(&mut input).unwrap();

    let mut values = input.trim().split_whitespace();

    let x = values.next()
        .and_then(|x| x.parse::<i32>().ok())
        .unwrap();

    let y = values.next()
        .and_then(|x| x.parse::<i32>().ok())
        .unwrap();

    if x < 0 || y < 0 || x > 432 ||  y > 468 {
        println!("fora");
    } else {
        println!("dentro");
    }
}
