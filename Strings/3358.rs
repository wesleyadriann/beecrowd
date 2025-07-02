use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines().skip(1) {
        let line = line.unwrap();

        let name_len = line.trim().len() - 1;
        let name = line.to_lowercase();

        let mut is_hard = false;
        for i in 1..name_len {
            let a = &name[i - 1..i];
            let b = &name[i..i + 1];
            let c = &name[i + 1..i + 2];
            if !check_is_vowels(a) && !check_is_vowels(b) && !check_is_vowels(c) {
                is_hard = true;
                break;
            }
        }

        if is_hard {
            println!("{} nao eh facil", line.trim());
        } else {
            println!("{} eh facil", line.trim());
        }
    }
}

fn check_is_vowels(letter: &str) -> bool {
    matches!(letter, "a" | "e" | "i" | "o" | "u")
}
