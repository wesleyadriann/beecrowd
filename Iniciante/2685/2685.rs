use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines() {
        let line = line.unwrap();

        let m: u32 = line.parse().unwrap();

        let result = match m {
            0..=89 => "Bom Dia!!",
            90..=179 => "Boa Tarde!!",
            180..=269 => "Boa Noite!!",
            270..=359 => "De Madrugada!!",
            _ => "Bom Dia!!"
        };

        println!("{}", result);
    }
}
