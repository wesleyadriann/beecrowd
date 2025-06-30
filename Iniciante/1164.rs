use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines().skip(1) {
        let line = line.unwrap();

        let n = line.trim().parse::<u32>().unwrap();
        let sum_of_div = sum_of_div(n);

        if n != sum_of_div || n == 1 {
            println!("{} nao eh perfeito", n);
        } else {
            println!("{} eh perfeito", n);
        }
    }
}

fn sum_of_div(n: u32) -> u32 {
    let mut result = 1;
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            result += i;
            if (i * i) != n {
                result += n / i;
            }
        }
        i += 1;
    }
    result
}
