use std::io::{self, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(io::stdin());

    for line in reader.lines().skip(1).step_by(2) {
        let line = line.unwrap();

        let mut values = line.split_whitespace().enumerate();

        let mut bigger = (0, 0);
        let mut second_bigger = (0, 0);

        while let Some((i, value)) = values.next() {
            let n: u32 = value.parse().unwrap();
            if n > bigger.1 {
                second_bigger = bigger;
                bigger = (i + 1, n);
            } else if n > second_bigger.1 {
                second_bigger = (i + 1, n);
            }
        }

        println!("{}", second_bigger.0);
    }
}
