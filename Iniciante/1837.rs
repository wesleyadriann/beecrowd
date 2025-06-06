use std::io;

fn main() {
  let mut input = String::with_capacity(12);

  io::stdin().read_line(&mut input).unwrap();

  let mut values = input.split_whitespace();

  let a: f64 = values.next()
    .and_then(|x| x.parse().ok())
    .unwrap();

  let b: f64 = values.next()
    .and_then(|x| x.parse().ok())
    .unwrap();

  let q;

  if b > 0.0 {
    q = (a / b).floor(); 
  } else {
    q = (a / b).ceil();
  }

  println!("{:.0} {:.0}", q, a - (b * q));
}