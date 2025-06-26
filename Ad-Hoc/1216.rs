use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let rader = BufReader::new(stdin.lock());

    let mut qtd = 0;
    let mut total = 0.0;

    for line in rader.lines().skip(1).step_by(2) {
        let line = line.unwrap();
        qtd += 1;
        total += line.trim().parse::<f64>().unwrap();
    }

    println!("{:.1}", total / qtd as f64);
}
