use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  loop {
    input.clear();

    let values: Vec<i32> = match stdin.read_line(&mut input) {
      Ok(_) => {
        let value = input.trim();
        if value.len() == 0 {
          break;
        }
        value.split_whitespace()
          .map(|x| x.parse().unwrap())
          .collect()
      }
      Err(_) => break,
    };

    println!("{}", 2 * values[0] * values[1]);
  }
}