use std::io;
use std::cmp::Ordering;

fn main() {
  const CORRECT: u32 = 2002;

  let stdin = io::stdin();

  loop {
    let mut input = String::new();

    let value: u32 = match stdin.read_line(&mut input) {
      Ok(_) => input.trim()
        .parse()
        .unwrap(),
      Err(_) => break,
    };

    match value.cmp(&CORRECT) {
      Ordering::Equal => {
        println!("Acesso Permitido");
        break;
      },
      _ => {
        println!("Senha Invalida");
      }
    }
  }
}