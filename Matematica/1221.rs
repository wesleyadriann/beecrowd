use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  stdin.read_line(&mut input).unwrap();

  let n = input.trim()
    .parse()
    .unwrap();
  
  for _ in 0..n {
    input.clear();
    stdin.read_line(&mut input).unwrap();

    let x: u32 = input.trim()
      .parse()
      .unwrap(); 

    let prime = is_prime(x);
    if prime {
      println!("Prime");
    } else {
      println!("Not Prime");
    }
  }
}

fn is_prime(n: u32) -> bool {
  if n <= 1 {
    return false;
  }
  if n == 2 {
    return true;
  }
  if n % 2 == 0 {
    return false;
  }
  for i in (3..=((n as f64).sqrt() as u32)).step_by(2) {
    if n % i == 0 {
      return false;
    }
  }
  true
}