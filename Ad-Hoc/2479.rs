use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();

    let line = lines.next().unwrap().unwrap();
    let n: usize = line.trim().parse().unwrap();

    let mut names: Vec<String> = Vec::with_capacity(n);
    let mut c = 0;
    let mut nc = 0;

    while let Some(line) = lines.next() {
        let line = line.unwrap();

        if &line[0..1] == "+" {
            c += 1;
        } else {
            nc += 1;
        }
        names.push(line[2..].trim().to_string());
    }

    names.sort();
    for name in names.iter() {
        println!("{}", name);
    }
    println!("Se comportaram: {} | Nao se comportaram: {}", c, nc);
}
