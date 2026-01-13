use std::io;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
 
    let mut pomekons: HashSet<String> = HashSet::with_capacity(151);

    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();

    let n: u32 = input.trim()
        .parse()
        .unwrap_or(0);

    for _ in 0..n {
        input.clear();
        stdin.read_line(&mut input).unwrap();

        let pomekon = input.trim()
            .to_lowercase();

        pomekons.insert(pomekon);
    }

    println!("Falta(m) {} pomekon(s).", 151 - pomekons.len());
}
