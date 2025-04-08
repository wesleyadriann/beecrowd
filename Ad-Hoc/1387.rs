use std::io;

fn main() {
  let stdin = io::stdin();
  loop {
    let mut input = String::new();

    let result = match stdin.read_line(&mut input) {
      Ok(_) => {
        let values: Vec<u8> = input.split_whitespace()
          .map(|x| x.parse().unwrap())
          .collect();
        values[0] + values[1]
      },
      Err(_) => break,
    };

    if result == 0 {
      break;
    }

    println!("{}", result);
  }
}