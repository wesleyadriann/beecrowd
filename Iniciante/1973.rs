use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let n: usize = match stdin.read_line(&mut input) {
    Ok(_) => input.trim()
      .parse()
      .unwrap(),
    Err(_) => return,
  };

  input.clear();
  let mut farms: Vec<u64> = match stdin.read_line(&mut input) {
    Ok(_) => input.split_whitespace()
      .map(|x| x.parse().unwrap())
      .collect(),
    Err(_) => return,
  };


  let mut stolen_farms = vec![false; n];
  let mut total = 0;
  let mut current = 0;
  loop {
    let value = match farms.get(current) {
      Some(value) => *value,
      None => 0,
    };

    if value == 0 {
      break;
    }

    farms[current] = farms[current] - 1;

    if !stolen_farms[current] {
      total += 1;
      stolen_farms[current] = true;
    }

    if value % 2 == 0 {
      if (current as i32) - 1 < 0 {
        break
      }
      current -= 1;
    } else {
      current += 1;
    }
  }

  let not_stolen: u64 = farms.iter().sum();
  println!("{} {}", total, not_stolen);
}