use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let n: u32 = match stdin.read_line(&mut input) {
    Ok(_) => input.trim().parse().unwrap(),
    Err(_) => return
  };

  for _ in 0..n {
    let mut input_w = String::new();

    let words: Vec<&str> = match stdin.read_line(&mut input_w) {
      Ok(_) => input_w.split_whitespace().collect(),
      Err(_) => break,
    };

    let (mut word_0, mut word_1) = (words[0].chars(), words[1].chars());

    let mut result = String::new();


    for _ in 0..input_w.len() {
      match word_0.next() {
        Some(letter) => result.push(letter),
        None => (),
      };
      match word_1.next() {
        Some(letter) => result.push(letter),
        None => (),
      };
    }

    println!("{}", result);
  }
}