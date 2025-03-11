use std::io;

fn main() {
  let mut input_n = String::new();

  io::stdin().read_line(&mut input_n).unwrap();

  let n: usize = input_n.trim().parse().unwrap();

  let mut words = Vec::with_capacity(n);

  for _ in 0..n {
    let mut input_word = String::new();
    io::stdin().read_line(&mut input_word).unwrap();

    let chars: Vec<char> = input_word.trim().chars().collect();
    let chars_len = chars.len();

    let mut new_word = Vec::new();

    for character in chars {
      if character.is_alphabetic() {
        let new_char = move_char_right(character);
        new_word.push(new_char);
      } else {
        new_word.push(character);
      }
    }

    new_word.reverse();

    for i in (chars_len / 2)..chars_len {
      let new_char = move_char_left(new_word[i]);
      new_word[i] = new_char;
    }

    let final_word: String = new_word.iter().collect();
    words.push(final_word);
  }

  println!("{}", words.join("\n"));
}

fn move_char_right(c: char) -> char {
  let ascii_value = c as u8; 
  let new_ascii_value = ascii_value + 3;  
  new_ascii_value as char
}

fn move_char_left(c: char) -> char {
  let ascii_value = c as u8;
  let new_ascii_value = ascii_value.wrapping_sub(1);
  new_ascii_value as char
}