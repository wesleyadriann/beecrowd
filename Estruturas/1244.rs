use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines().skip(1) {
        let line = line.unwrap();

        let mut values = line.split_whitespace().collect::<Vec<&str>>();

        values.sort_by(|a, b| b.len().cmp(&a.len()));
        println!("{}", values.join(" "));
    }
}
