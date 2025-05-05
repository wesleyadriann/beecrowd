use std::io;

fn main() {
  let stdin = io::stdin();

  let mut first = true;
  loop {  
    let mut input_n = String::new();
    let n: u32 = match stdin.read_line(&mut input_n) {
      Ok(_) => input_n.trim()
        .parse()
        .unwrap(),
      Err(_) => break,
    };

    if n == 0 {
      break
    }

    if first {
      first = false;
    } else {
      println!("");
    }

    let mut phrases = Vec::new();

    let mut bigger = 0;
    for _ in 0..n {
      let mut input_phrase = String::new();

      match stdin.read_line(&mut input_phrase) {
        Ok(_) => {
          let clean = input_phrase.split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");
          if clean.len() > bigger {
            bigger = clean.len()
          }
          phrases.push(clean);
        },
        Err(_) => continue,
      };
    }

    let result: Vec<String> = phrases.iter()
      .map(|s| {
        let pad_left = bigger - s.len();
        format!("{:width$}{}", "", s, width=pad_left)
      })
      .collect();
    println!("{}", result.join("\n"));
  }  
}