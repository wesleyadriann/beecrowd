use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let next_l = lines.next().unwrap().unwrap();

        let mut values = line.split_whitespace();
        let n = values.next().and_then(|x| x.parse::<usize>().ok()).unwrap();
        let r = values.next().and_then(|x| x.parse::<usize>().ok()).unwrap();

        if n == r {
            println!("*");
            continue;
        }

        let mut back = vec![false; n];

        for item in next_l.split_whitespace() {
            let item = item.parse::<usize>().unwrap();
            back[item - 1] = true;
        }

        for (i, &value) in back.iter().enumerate() {
            if !value {
                print!("{} ", i + 1);
            }
        }
        println!("");
    }
}
