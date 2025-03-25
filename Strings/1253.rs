use std::io;

fn main() {
  let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  let stdin = io::stdin();

  let mut input = String::new();

  stdin.read_line(&mut input).unwrap();

  let n = input.trim()
    .parse()
    .unwrap();

  for _ in 0..n {
    let mut phrase = String::new();
    stdin.read_line(&mut phrase).unwrap();

    input.clear();
    stdin.read_line(&mut input).unwrap();

    let shift: u16 = input.trim()
      .parse()
      .unwrap();

    let characters = phrase.trim().chars();

    let mut result = String::new();
    for character in characters {
      let position = alphabet.find(character).unwrap() as i16;

      let curr_position = position - (shift) as i16;
      let index: usize;
      if curr_position < 0 {
        index = (curr_position + 26) as usize;
      } else {
        index = curr_position as usize
      }

      let new_char = alphabet.chars()
        .nth(index)
        .unwrap();
      result.push(new_char);
    }

    println!("{}", result);
  }
}
