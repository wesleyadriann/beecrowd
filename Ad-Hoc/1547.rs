use std::io::{self, BufRead, BufReader};

fn main() {
    let mut reader = BufReader::new(io::stdin());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let n: u32 = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();

        let mut qts = input.split_whitespace();
        let qt = qts.next().and_then(|x| x.parse::<usize>().ok()).unwrap();
        let s = qts.next().and_then(|x| x.parse::<i32>().ok()).unwrap();

        input.clear();
        reader.read_line(&mut input).unwrap();

        let mut values = input.split_whitespace();

        let mut diff: Vec<i32> = Vec::with_capacity(qt);

        while let Some(value) = values.next() {
            let value: i32 = value.parse().unwrap();
            diff.push((value - s).abs());
        }

        let mut result_diff = i32::MAX;
        let mut result_index = 0;

        for (i, &value) in diff.iter().enumerate() {
            if value < result_diff {
                result_index = i;
                result_diff = value;
            }
        }

        println!("{}", result_index + 1);
    }
}
