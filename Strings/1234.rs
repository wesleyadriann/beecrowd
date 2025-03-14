use std::io;

fn main() {
  let mut words = String::new();

  loop {
    let mut curr = String::new();

    io::stdin().read_line(&mut curr).unwrap();

    if curr.trim().is_empty() {
      break;
    } 

    words.push_str(&curr);
  }

  let chars: Vec<char> = words.trim().chars().collect();
  let mut final_word = String::new();
  let mut to_uppercase = true;
  for character in chars {
    let mut final_char: char = character.to_lowercase().collect::<Vec<char>>()[0];
    
    
    if character == 0xA as char {
      to_uppercase = true;
    }
    
    if character.is_alphabetic() {
      if to_uppercase {
        final_char = final_char.to_uppercase().collect::<Vec<char>>()[0];
      }
      to_uppercase = !to_uppercase;
    }
    final_word.push(final_char);
  }

  println!("{}",final_word);
}