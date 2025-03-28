use std::io;

fn main() {
  let stdin = io::stdin();
  let mut input = String::with_capacity(50);
  loop {
    input.clear();
    stdin.read_line(&mut input).unwrap();

    let phrase: Vec<&str> = input.trim() 
      .split_whitespace()
      .collect();

    if phrase[0] == "*" as &str {
      break;
    }

    let mut result = true;
    let mut prev_letter: Option<char> = None; 
    for word in phrase {
      let curr_letter = word.to_uppercase()
        .chars()
        .next()
        .unwrap();
      match prev_letter {
        Some(prev) => {
          if curr_letter != prev {
            result = false;
            break;
          }
        },
        None => {
          prev_letter = Some(curr_letter)
        }
      }
    }

    if result {
      println!("Y");
    } else {
      println!("N");
    }

  }
}