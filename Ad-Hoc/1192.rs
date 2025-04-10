use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let n = match stdin.read_line(&mut input) {
    Ok(_) => input.trim()
      .parse()
      .unwrap(),
    Err(_) => return,
  };

  for _ in 0..n {
    let mut input_value = String::new();

    let values: Vec<char> = match stdin.read_line(&mut input_value) {
      Ok(_) => input_value.trim().chars().collect(),
      Err(_) => break,
    };

    let (n1, cha, n2): (i32, char, i32) = (
      values[0].to_digit(10).unwrap() as i32,
      values[1],
      values[2].to_digit(10).unwrap() as i32
    );

    if n1 == n2 {
      println!("{}", n1 * n2);
      continue
    }

    if cha.is_uppercase() {
      println!("{}", n2 - n1);
    } else {
      println!("{}", n1 + n2);
    }
  }
}