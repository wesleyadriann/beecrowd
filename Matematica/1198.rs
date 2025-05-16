use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();
  loop {
    input.clear();

    let values: Vec<i64> = match stdin.read_line(&mut input) {
      Ok(_) => input.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect(),
      Err(_) => break,
    };

    if values.len() == 0 {
      break;
    }

    println!("{}", (values[0] - values[1]).abs());
  }
}