use std::io;

fn main() {
  let mut input = String::new();

  let values: Vec<u32> = match io::stdin().read_line(&mut input) {
    Ok(_) => input.split_whitespace()
      .map(|x| x.parse().unwrap())
      .collect(),
    Err(_) => return,
  };

  let (c, p, f) = (values[0], values[1], values[2]);

  if c * f <= p {
    println!("S");
  } else {
    println!("N");
  }
}