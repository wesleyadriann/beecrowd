use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();

    let n = lines.next().unwrap()
        .unwrap();
    let n: u32 = n.parse().unwrap();

    for _ in 0..n {
        let mut p1 = 0;
        let mut p2 = 0;

        let values = lines.by_ref().take(6);

        for (i, line) in values.enumerate() {
            let line = line.unwrap();
            let mut line_values = line.split_whitespace();

            let x = line_values.next().unwrap();
            let x: u32 = x.parse().unwrap();

            let d = line_values.next().unwrap();
            let d: u32 = d.parse().unwrap();

            if i < 3 {
                p1 += x * d;
            } else {
                p2 += x * d;
            }
        }

        if p1 > p2 {
            println!("JOAO");
        } else {
            println!("MARIA");
        }
    }

}
