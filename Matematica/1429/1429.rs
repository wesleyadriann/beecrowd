use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim() == "0" {
            continue;
        }

        let mut fac_curr = line.trim().len();
        let line_chars = line.chars();

        let mut result = 0;

        for curr_char in line_chars {
            let fac = (1..=fac_curr).product::<usize>() as u32;
            let n: u32 = curr_char.to_digit(10).unwrap();

            result += fac * n;

            fac_curr -= 1;
        }

        println!("{}", result);
    }
}
