use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();
    loop {
        let n = lines.next()
            .unwrap()
            .and_then(|x| Ok(x.parse::<u32>().unwrap()))
            .unwrap();

        if n == 0 {
            break;
        }

        let mut planet = String::with_capacity(52);
        let mut year = u32::MAX;

        for _ in 0..n {
            let line = lines.next()
                .unwrap()
                .unwrap();

            let mut values = line.split_whitespace();

            let name = values.next();
            let a = values.next()
                .and_then(|x| x.parse::<u32>().ok())
                .unwrap();
            let t = values.next()
                .and_then(|x| x.parse::<u32>().ok())
                .unwrap();

            if a - t < year {
                year = a - t;
                planet = name.unwrap().to_string();
            }
        }

        println!("{}", planet);
    }
}
