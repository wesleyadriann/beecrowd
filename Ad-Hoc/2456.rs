use std::io;

fn main() {
    let stdin = io::stdin();

    let mut input = String::with_capacity(12);

    stdin.read_line(&mut input).expect("Failed to read line");

    let mut cards = input.split_whitespace();

    let mut is_asc = true;
    let mut is_desc = true;

    let mut prev_card = cards.next().and_then(|x| x.parse::<u32>().ok()).unwrap();
    while let Some(card) = cards.next().and_then(|x| x.parse::<u32>().ok()) {
        if card > prev_card {
            is_desc = false;
        } else if card < prev_card {
            is_asc = false;
        }
        prev_card = card;
    }

    if is_asc {
        println!("C");
    } else if is_desc {
        println!("D");
    } else {
        println!("N");
    }
}
