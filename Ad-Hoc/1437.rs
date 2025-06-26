use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines().skip(1).step_by(2) {
        let line = line.unwrap();

        let mut degree: i32 = 90;

        let mut values = line.trim().chars();
        while let Some(value) = values.next() {
            if value == 'D' {
                degree -= 90;
            } else {
                degree += 90;
            }
        }

        let result = ((degree % 360) + 360) % 360;
        match result {
            0 => println!("L"),
            90 => println!("N"),
            180 => println!("O"),
            270 => println!("S"),
            _ => {}
        }
    }
}
