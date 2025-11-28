use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines().skip(1) {
        let line = line.unwrap();

        let mut values = line.split_whitespace();

        let h = values.next().unwrap();
        let m = values.next().unwrap();
        let o = values.next().unwrap();

        let o = if o == "0" { "A porta fechou!" } else { "A porta abriu!" };

        println!("{:0>2}:{:0>2} - {}", h, m, o);
    }
}
