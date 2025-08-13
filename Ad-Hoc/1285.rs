use std::io::{self, BufRead, BufReader};
// use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines() {
        let line = line.unwrap();

        let mut values = line.split_whitespace();

        let start = values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();
        let end = values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();

        let mut result = 0;

        // let mut number_set = HashSet::new();
        for n in start..=end {
            let has_duplicates = has_duplicate_digits(n);
            if !has_duplicates {
                result += 1;
            }
             // let value = n.to_string();

            // for v in value.chars() {
                // number_set.insert(v);
            // }
            // if number_set.len() == value.len() {
            //     result += 1;
            // }
            // number_set.clear();
        }
        println!("{}", result);
    }
}

fn has_duplicate_digits(mut n: u32) -> bool {
    let mut seen = [false; 10];
    while n > 0 {
        let digit = (n % 10) as usize;
        if seen[digit] {
            return true;
        }
        seen[digit] = true;
        n /= 10;
    }
    false
}

