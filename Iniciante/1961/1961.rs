use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();
    let p = values.next().and_then(|x| x.parse::<i32>().ok()).unwrap();

    input.clear();
    stdin.read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let mut prev = values.next().and_then(|x| x.parse::<i32>().ok()).unwrap();
    let mut win = true;
    while let Some(value) = values.next() {
        let value = value.parse::<i32>().unwrap();

        let low = prev - p;
        let max = prev + p;
        if value < low || value > max {
            win = false;
            break;
        }

        prev = value;
    }

    if win {
        println!("YOU WIN");
    } else {
        println!("GAME OVER");
    }
}
