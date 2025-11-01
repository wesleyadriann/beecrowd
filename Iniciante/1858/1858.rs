use std::io;


fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    input.clear();

    stdin.read_line(&mut input).unwrap();

    let mut small = u32::MAX;
    let mut small_index = 0;

    let mut values = input.split_whitespace().enumerate();

    while let Some((i, value)) = values.next() {
        let value = value.parse::<u32>().unwrap();

        if value < small {
            small = value;
            small_index = i;
        }
    }

    println!("{}", small_index + 1);
}
