use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut i = 1;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut values = line.split_whitespace();

        let x = values.next().and_then(|x| x.parse::<f64>().ok()).unwrap();
        let y = values.next().and_then(|x| x.parse::<f64>().ok()).unwrap();

        let result: f64 = (y / x - 1.0) * 100.0;


        println!("Projeto {}:", i);
        println!("Percentual dos juros da aplicacao: {:.2} %\n", result);

        i += 1;
    }
}
