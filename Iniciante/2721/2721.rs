use std::io;

fn main() {
    const NAMES: [&str; 9] = [
        "Dasher",
        "Dancer",
        "Prancer",
        "Vixen",
        "Comet",
        "Cupid",
        "Donner",
        "Blitzen",
        "Rudolph"
    ];
    let mut input = String::with_capacity(20);

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let mut values = input.split_whitespace();

    let mut total: u32 = 0;

    while let Some(value) = values.next() {
        let value = value.parse::<u32>().unwrap();
        total += value;
    }

    let mut index = (total % 9) as usize;

    if index == 0  {
        index = 9;
    }

    println!("{}", NAMES[index - 1]);

}
