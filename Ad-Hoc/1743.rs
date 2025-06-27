use std::io;

fn main() {
    let stdin = io::stdin();

    let mut input_x = String::with_capacity(12);
    let mut input_y = String::with_capacity(12);

    stdin.read_line(&mut input_x).unwrap();

    let mut values_x = input_x.split_whitespace();

    stdin.read_line(&mut input_y).unwrap();

    let mut values_y = input_y.split_whitespace();

    let mut result = "Y";

    for _ in 0..5 {
        let x = values_x.next().and_then(|x| x.parse::<u8>().ok()).unwrap();
        let y = values_y.next().and_then(|y| y.parse::<u8>().ok()).unwrap();

        if x ^ y == 0 {
            result = "N";
            break;
        }
    }

    println!("{}", result);
}
