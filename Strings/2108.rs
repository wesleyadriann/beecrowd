use std::io;

fn main() {
  let stdin = io::stdin();
  let mut input = String::new();

  let mut bigger_word = String::with_capacity(100);
  loop {
    input.clear();
    
    let words: Vec<&str> = match stdin.read_line(&mut input) {
      Ok(_) => input.trim().split_whitespace().collect(),
      Err(_) => break,
    };

    if words[0] == "0" {
      break;
    }

    let mut result: Vec<String> = Vec::new();

    for word in words {
      let word_len = word.len();
      
      if word_len >= bigger_word.len() {
        bigger_word.clear();
        bigger_word.push_str(word);
      }
      result.push(word_len.to_string());
    }

    println!("{}", result.join("-"));
  }
  println!("\nThe biggest word: {}", bigger_word);
}