use std::collections::VecDeque;
use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim() == "0" {
            break;
        }

        let n: u32 = line.trim().parse().unwrap();

        let mut cartas: VecDeque<u32> = (1..=n).collect();
        let mut descarte: Vec<String> = Vec::with_capacity(cartas.len());

        while cartas.len() > 1 {
            let carta = cartas.pop_front().unwrap();
            descarte.push(carta.to_string());

            let carta = cartas.pop_front().unwrap();
            cartas.push_back(carta);
        }

        println!("Discarded cards: {}", descarte.join(", "));
        println!("Remaining card: {}", cartas[0]);
    }
}
