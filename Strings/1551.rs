use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines().skip(1) {
        let line = line.unwrap();

        let mut alphabet = [0; 26];

        let chars = line.chars();
        for char in chars {
            let c = char as i32;
            let index = (c - 97) as usize;
            if index >= 0 && index < 26 {
                alphabet[index] = 1;
            }
        }

        let result: u32 = alphabet.iter().sum::<u32>();

        if result < 14 {
            println!("frase mal elaborada");
        } else if result < 26 {
            println!("frase quase completa");
        } else {
            println!("frase completa");
        }

    }
}
