use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let input_trimmed = input.trim();
    let input_rev: String = input.trim().chars().rev().collect();

    if input_trimmed == input_rev {
        println!("A frase [{}] eh palindrome", input_trimmed);
    } else {
        println!("A frase [{}] nao eh palindrome", input_trimmed);
    }
}
