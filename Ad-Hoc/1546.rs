use std::io::{self, BufRead, BufReader};

fn main() {
    let mut reader = BufReader::new(io::stdin());

    let mut input = String::with_capacity(8);

    reader.read_line(&mut input).unwrap();

    let n = input.trim().parse::<u16>().unwrap();

    for _ in 0..n {
        input.clear();

        reader.read_line(&mut input).unwrap();

        let k = input.trim().parse::<u16>().unwrap();

        for _ in 0..k {
            input.clear();

            reader.read_line(&mut input).unwrap();

            let result = match input.trim() {
                "1" => "Rolien",
                "2" => "Naej",
                "3" => "Elehcim",
                _ => "Odranoel",
            };

            println!("{}", result);
        }
    }
}
