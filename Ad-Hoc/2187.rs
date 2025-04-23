use std::io;

fn main() {
  let stdin = io::stdin();
  
  let mut i = 0;

  loop {
    if i > 0 {
      println!();
    }

    let mut input = String::new();
    let v: u32 = match stdin.read_line(&mut input) {
      Ok(_) => input.trim()
        .parse()
        .unwrap(),
      Err(_) => break,
    };

    if v == 0 {
      break;
    }

    let mut rest = v % 50;
    let bit_50 = v / 50;

    let bit_10 = rest / 10;
    rest = rest % 10;

    let bit_5 = rest / 5;
    rest = rest % 5;
  
    i += 1;
    println!("Teste {}", i);
    println!("{} {} {} {}", bit_50, bit_10, bit_5, rest);
    
  }
}