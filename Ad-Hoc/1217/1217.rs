use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();

    let n = lines.next()
        .unwrap()
        .and_then(|x| Ok(x.parse::<u32>().unwrap()))
        .unwrap();

    let mut total = 0;
    let mut total_value = 0_f64;

    for i in 0..n {
        let v = lines.next()
            .unwrap()
            .and_then(|x| Ok(x.parse::<f64>().unwrap()))
            .unwrap();

        let line = lines.next()
            .unwrap()
            .unwrap();

        let qtd = line.split_whitespace().count();

        total += qtd;
        total_value += v;

        println!("day {}: {} kg", i + 1, qtd);
    }

    let n = n as f64;
    println!("{:.2} kg by day", total as f64 / n);
    println!("R$ {:.2} by day", total_value / n);

}
