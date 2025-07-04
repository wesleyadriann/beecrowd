use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();

    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();

    let op_line: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let op = lines.next().unwrap().unwrap();

    let mut matrix = [0.0; 144];

    let mut index = 0;
    for line in lines {
        let line = line.unwrap();

        let value = line.parse::<f64>().unwrap();
        matrix[index] = value;
        index += 1;
    }

    let start = 12 * op_line;
    let end = start + 12;

    let mut result = matrix[start..end].iter().sum::<f64>();
    if op == "M" {
        result /= 12.0;
    }
    println!("{:.1}", result);
}
