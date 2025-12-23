use std::io;

fn main() {
    let mut input = String::with_capacity(21);

    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();

    if input.len() >= 10 {
        println!("palavrao");
    } else {
        println!("palavrinha");
    }
}
