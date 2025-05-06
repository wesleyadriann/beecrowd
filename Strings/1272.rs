use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  stdin.read_line(&mut input).unwrap();

  let n: u32 = input.trim()
    .parse()
    .unwrap_or(0);

  for _ in 0..n {
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();

    let words: Vec<&str> = input.split_whitespace().collect();

    let mut result = String::new();

    for word in words {
      let l = word.chars()
        .next()
        .unwrap();
      result.push(l);
    }

    println!("{}", result);
  }
}