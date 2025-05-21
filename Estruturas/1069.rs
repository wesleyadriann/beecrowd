use std::io::{
  self,
  BufRead,
  BufReader
};

fn main() {
  let mut input = String::new();

  let mut reader = BufReader::new(io::stdin());

  reader.read_line(&mut input).unwrap();
  
  let n: u32 = input.trim()
    .parse()
    .unwrap();

  for _ in 0..n {
    input.clear();
    reader.read_line(&mut input).unwrap();

    let mut chars = input.trim()
      .chars()
      .filter(|&c| c == '<' || c == '>')
      .collect::<Vec<_>>();

    let mut index = 0;
    let mut result = 0;
    loop {
      if index + 1 >= chars.len() {
        break;
      }
      let curr = chars[index];
      let next = chars[index + 1];

      if curr == '<' && next == '>' {
        chars.remove(index);
        chars.remove(index);
        result += 1;
        index = 0;
      } else {
        index += 1;
      }
    }
    println!("{}", result);
  }
}

