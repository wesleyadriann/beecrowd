use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines().skip(1) {
        let line = line.unwrap();
        let n = line.trim().parse::<u32>().unwrap();
        let fib = fib(n);

        println!("Fib({}) = {}", n, fib);
    }
}

fn fib(n: u32) -> u64 {
    if n == 0 || n == 1 {
        return n as u64;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 1..n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}
