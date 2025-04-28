use std::io;

fn main() {
  let stdin = io::stdin();

  let mut notes: Vec<f64> = Vec::new();

  while notes.len() < 2 {
    let mut input = String::new();

    let value: f64 = match stdin.read_line(&mut input) {
      Ok(_) => input.trim()
        .parse()
        .unwrap(),
      Err(_) => continue,
    };

    if value < 0.0 || value > 10.0 {
      println!("nota invalida");
    } else {
      notes.push(value)
    }
  }

  let result: f64 = (notes[0] + notes[1]) / 2.0;
  println!("media = {:.2}", result);
}