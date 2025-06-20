use std::io::{self, BufRead, BufReader};

fn main() {
    let mut reader = BufReader::new(io::stdin());

    let mut input = String::with_capacity(100);

    reader.read_line(&mut input).unwrap();

    let n: u32 = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();

        reader.read_line(&mut input).unwrap();
        let m = input.trim().parse::<usize>().unwrap();
        let mut fila: Vec<(u32, u32)> = Vec::with_capacity(m);

        input.clear();
        reader.read_line(&mut input).unwrap();

        let mut values = input.split_whitespace();

        let mut i = 0;
        while let Some(value) = values.next().and_then(|v| v.parse::<u32>().ok()) {
            fila.push((i, value));
            i += 1;
        }

        fila.sort_by(|(_, a), (_, b)| b.cmp(a));

        let mut result = 0;

        for (i, &(original_i, _)) in fila.iter().enumerate() {
            if original_i == i as u32 {
                result += 1;
            }
        }

        println!("{}", result);
    }
}
