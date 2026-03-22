use std::io::{self, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(io::stdin());

    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();

        if line.trim() == "0" {
            break;
        }

        let mut values = line.split_whitespace();

        let q = values.next()
            .and_then(|x| x.parse::<f64>().ok())
            .unwrap();
        let d = values.next()
            .and_then(|x| x.parse::<f64>().ok())
            .unwrap();
        let p = values.next()
            .and_then(|x| x.parse::<f64>().ok())
            .unwrap();

        let result = ((q * d * p) / (p - q)).floor();

        let suffix = if result > 1.0 { "s" } else { "" };

        println!("{:.0} pagina{}", result, suffix);
    }
}
