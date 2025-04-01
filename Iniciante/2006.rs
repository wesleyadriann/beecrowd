use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::with_capacity(10);

  let t: u16 = match stdin.read_line(&mut input) {
    Ok(_) => input.trim().parse().unwrap(),
    Err(_) => return
  };

  input.clear();
  let r: Vec<u16> = match stdin.read_line(&mut input)  {
    Ok(_) => input.split_whitespace()
      .map(|x| x.parse().unwrap())
      .collect(),
    Err(_) => return,
  };

  let mut result: u16 = 0;
  for i in r {
    if i == t {
      result += 1;
    }
  }

  println!("{}", result);

}