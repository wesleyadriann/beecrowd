use std::{
  io,
  cmp::Ordering,
};

fn main() {
  let mut input = String::new();

  let (p1, c1, p2, c2) = match io::stdin().read_line(&mut input) {
    Ok(_) => {
      let values: Vec<u16> = input.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
      (values[0], values[1], values[2],  values[3])
    },
    Err(_) => return,
  };

  let left = p1 * c1;
  let right = p2 * c2;

  match left.cmp(&right) {
    Ordering::Equal => println!("0"),
    Ordering::Less => println!("1"),
    Ordering::Greater => println!("-1"),
  }
}