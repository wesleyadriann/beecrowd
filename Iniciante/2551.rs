use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let n = line.parse::<usize>().unwrap();

        let mut taked = lines.by_ref().take(n);

        let mut v_m = 0f64;
        let mut i = 1;

        while let Some(taked_line) = taked.next() {
            let taked_line = taked_line.unwrap();
            let mut values = taked_line.split_whitespace();

            let t = values.next()
                .and_then(|x| x.parse::<f64>().ok())
                .unwrap();

            let d = values.next()
                .and_then(|x| x.parse::<f64>().ok())
                .unwrap();

            let v_mi = d / t;

            if v_mi > v_m {
                println!("{}", i);
                v_m = v_mi;
            }

            i += 1;
        }
    }
}
