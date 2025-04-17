use std::io;

fn main() {
  let stdin = io::stdin();

  loop {
    let mut input = String::new();

    let n: u32 = match stdin.read_line(&mut input) {
      Ok(_) => input.trim()
        .parse()
        .unwrap(),
      Err(_) => break,
    };

    if n == 0 {
      break;
    }

    let result = (n * (n + 1) * (2 * n + 1)) / 6;
    println!("{}", result);
  }
}