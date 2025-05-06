use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let n = match stdin.read_line(&mut input) {
    Ok(_) => input.trim()
      .parse()
      .unwrap_or(0),
    Err(_) => return,
  };

  for _ in 0..n {
    input.clear();
    let word = match stdin.read_line(&mut input) {
      Ok(_) => input.trim().chars(),
      Err(_) => break
    };

    if word.clone().count() > 3 {
      println!("3");
      continue;
    }
    
    let is_1 = {
      let mut count = 0;
      let mut iter = word;
      if iter.next().unwrap() == 'o' {
        count += 1;
      }
      if iter.next().unwrap() == 'n' {
        count += 1;
      }
      if iter.next().unwrap() == 'e' {
        count += 1;
      }
      count >= 2
    };

    if is_1 {
      println!("1");
    } else {
      println!("2");
    }
  }
}