use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let n: u32 = match stdin.read_line(&mut input) {
    Ok(_) => input.trim()
      .parse()
      .unwrap(),
    Err(_) => return,
  };

  for _ in 0..n {
    input.clear();

    let values: Vec<u32> = match stdin.read_line(&mut input) {
      Ok(_) => input.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect(),
      Err(_) => break,
    };

    let (start, end) = if values[0] > values[1] {
      (values[1] + 1, values[0])
    } else {
      (values[0] + 1, values[1])
    };

    let mut result = 0;
    for i in start..end {
      if i % 2 != 0 {
        result += i;
      }
    }
    println!("{}", result);
  }
}