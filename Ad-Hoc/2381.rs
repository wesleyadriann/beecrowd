use std::io::{self, BufRead, BufReader};

fn main() {
    let mut reader = BufReader::new(io::stdin());

    let mut input = String::with_capacity(32);

    reader.read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let n: usize = values.next().and_then(|x| x.parse::<usize>().ok()).unwrap();
    let k: usize = values.next().and_then(|x| x.parse::<usize>().ok()).unwrap();

    let mut alunos: Vec<String> = Vec::with_capacity(n);

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();

        let aluno = input.trim().to_string();
        alunos.push(aluno);
    }

    alunos.sort();

    println!("{}", alunos[k - 1]);
}
