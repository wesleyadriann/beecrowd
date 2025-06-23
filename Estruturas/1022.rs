use std::io::{ self, BufRead, BufReader };

fn main() {
    let stdin = io::stdin();
    stdin.read_line(&mut String::with_capacity(4)).unwrap();


    let reader = BufReader::new(stdin.lock());

    for line in reader.lines() {
        let line = line.unwrap();

        if line.trim().is_empty() {
            break;
        }

        let mut values = line.split_whitespace();

        let n1: i32 = values.next().and_then(|x| x.parse().ok()).unwrap();
        values.next();
        let d1: i32 = values.next().and_then(|x| x.parse().ok()).unwrap();

        let op = values.next().unwrap();

        let n2: i32 = values.next().and_then(|x| x.parse().ok()).unwrap();
        values.next();
        let d2: i32 = values.next().and_then(|x| x.parse().ok()).unwrap();

        let (n, d) = match op {
            "+" => (n1 * d2 + n2 * d1, d1 * d2),
            "-" => (n1 * d2 - n2 * d1, d1 * d2),
            "*" => (n1 * n2, d1 * d2),
            _ => (n1 * d2, n2 * d1),
        };

        let mut rn = n;
        let mut rd = d; 
        loop {
            let result_mdc = mdc(rn, rd);
            if result_mdc == 1 {
                break;
            }
            rn /= result_mdc;
            rd /= result_mdc;
        }

        println!("{}/{} = {}/{}", n, d, rn, rd);
    }
}

fn mdc(n1: i32, n2: i32) -> i32{
  let mut a = n1;
  let mut b = n2;
  while b != 0 {
    let r = a % b;
    a = b;
    b = r;
  }
  return a.abs();
}

