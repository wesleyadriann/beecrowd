use std::io;

fn main() {
  loop {
    let mut input = String::new();
  
    let numbers: Vec<u32> = match io::stdin().read_line(&mut input) {
      Ok(_) => input.split_whitespace()
          .map(|x| x.parse().unwrap())
          .collect(),
      Err(_) => break
    };

    let (x, y) = (numbers[0], numbers[1]);

    if x == y {
      break;
    }

    if x > y {
      println!("Decrescente");
    } else {
      println!("Crescente");
    }
  }
}