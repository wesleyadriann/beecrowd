use std::io;

fn main() {
  let mut input = String::new();

  let n: u32 = match io::stdin().read_line(&mut input) {
    Ok(_) => input.trim()
      .parse()
      .unwrap(),
    Err(_) => return,
  };

  for i in 1..=n {
    println!("{} {} {}", i, i.pow(2), i.pow(3));
  }
}