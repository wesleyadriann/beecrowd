use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};
use std::iter::FromIterator;

fn main() {
    let mut reader = BufReader::new(io::stdin());

    let mut input = String::with_capacity(18);

    reader.read_line(&mut input).unwrap();

    let n = input.trim().parse::<u32>().unwrap();

    for i in 0..n {
        input.clear();

        let mut sudoku: Vec<u32> = Vec::with_capacity(81);

        for _ in 0..9 {
            input.clear();

            reader.read_line(&mut input).unwrap();

            let mut values = input.trim().split_whitespace();

            while let Some(value) = values.next().and_then(|v| v.parse::<u32>().ok()) {
                sudoku.push(value);
            }
        }

        let mut result = "SIM";

        for j in 0..9 {
            let line = check_line(&sudoku, j);
            let column = check_column(&sudoku, j);
            let square = check_square(&sudoku, j);

            if !line || !column || !square {
                result = "NAO";
                break;
            }
        }

        println!("Instancia {}", i + 1);
        println!("{}\n", result);
    }
}

fn check_line(sudoku: &Vec<u32>, line: usize) -> bool {
    let start = line * 9;
    let end = start + 9;
    let hash_set: HashSet<&u32> = HashSet::from_iter(sudoku[start..end].iter());
    hash_set.len() == 9
}

fn check_column(sudoku: &Vec<u32>, column: usize) -> bool {
    let hash_set: HashSet<&u32> = HashSet::from_iter(sudoku.iter().skip(column).step_by(9));

    hash_set.len() == 9
}

fn check_square(sudoku: &Vec<u32>, square: usize) -> bool {
    let start = square * 3 % 9 + ((square / 3) * 27);
    let end = start + 3;

    let line_1 = &sudoku[start..end];
    let line_2 = &sudoku[start + 9..end + 9];
    let line_3 = &sudoku[start + 18..end + 18];

    let mut hash_set: HashSet<u32> = HashSet::with_capacity(9);

    hash_set.extend(line_1);
    hash_set.extend(line_2);
    hash_set.extend(line_3);

    hash_set.len() == 9
}
