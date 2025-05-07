use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  stdin.read_line(&mut input).unwrap();
  let n: u32 = input.trim()
    .parse()
    .unwrap_or(0);

  for _ in 0..n {
    input.clear();
    stdin.read_line(&mut input).unwrap();
    let values: Vec<i32> = input.split_whitespace()
      .map(|x| x.parse().unwrap())
      .collect();

    let (x, y) = (values[0], values[1]);

    let r_result = r(x, y);
    let b_result = b(x, y);
    let c_result = c(x, y);

    if r_result > b_result && r_result > c_result {
      println!("Rafael ganhou");
    } else if b_result > r_result && b_result > c_result {
      println!("Beto ganhou");
    } else {
      println!("Carlos ganhou");
    }
  }
}

fn r(x: i32, y: i32) -> i32 {
  (3 * x).pow(2) + (y).pow(2)
}

fn b(x: i32, y: i32) -> i32 {
  2 * (x.pow(2)) + (5 * y).pow(2)
}

fn c(x: i32, y: i32) -> i32 {
  -100 * x + y.pow(3)
}