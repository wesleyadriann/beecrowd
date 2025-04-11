use std::io;

fn main() {
  let stdin = io::stdin();
  let mut input = String::new();

  loop {
    input.clear();
    let n = match stdin.read_line(&mut input) {
      Ok(_) => {
        if input.trim().len() == 0 {
          break;
        }
        input.trim().chars()
      },
      Err(_) => break,
    };

    let mut miss_open = false;
    let mut result = Vec::new();
    for cha in n {
      match cha {
        '(' => {
          result.push('(');
        },
        ')' => {
          if result.len() > 0 {
            result.pop();
          } else {
            miss_open = true;
          }
        },
        _ => continue,
      }
    }
    if result.len() > 0 || miss_open {
      println!("incorrect");
    } else {
      println!("correct");
    }
  }
}