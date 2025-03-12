use std::io;

fn main() {
  loop {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let values: Vec<&str> = input.trim()
      .split_whitespace()
      .collect();

    let d = values[0].parse::<u16>().unwrap();
    let n = match values[1].parse::<u128>() {
      Ok(num) => num,
      Err(_) => 1,
    };

    if d == 0 && n == 0 {
      break;
    }

    let new_value: String = values[1].replace(values[0], "");
    let new_value = new_value.trim_start_matches('0');

    if new_value.len() == 0 {
      println!("0");
    } else {
      println!("{}", new_value);
    }
  }
}