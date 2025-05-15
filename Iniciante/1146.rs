use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  loop {
    input.clear();
    stdin.read_line(&mut input).unwrap();

    let n: u32 = input.trim()
      .parse()
      .unwrap();

    if n == 0 {
      break
    }

    for i in 1..=n {
      if i == n {
        println!("{}", i);
      } else {
        print!("{} ", i);
      }
    }
  }
}