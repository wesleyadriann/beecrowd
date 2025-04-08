use std::io;

fn main() {
  let mut input = String::new();

  let n: u16 = match io::stdin().read_line(&mut input) {
    Ok(_) => input.trim()
      .parse()
      .unwrap(),
    Err(_) => return,
  };

  let result: char = match n {
    86..=100 => 'A',
    61..=85 => 'B',
    36..=60 => 'C',
    1..=35 => 'D',
    _ => 'E',
  };

  println!("{}", result);
}