use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let (n, s) = match stdin.read_line(&mut input) {
    Ok(_) => {
      let temp: Vec<i32> = input.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
      (temp[0], temp[1])
    },
    Err(_) => return,
  };


  let mut result = s;
  let mut curr = s;

  let mut input_n = String::new();
  for _ in 0..n {
    input_n.clear();
    let value: i32 = match stdin.read_line(&mut input_n) {
      Ok(_) => input_n.trim()
        .parse()
        .unwrap(),
      Err(_) => break,
    };
    curr += value;
    if curr < result {
      result = curr;
    }
  }
  println!("{}", result);

}