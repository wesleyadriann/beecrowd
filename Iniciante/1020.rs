use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let mut d = input.trim().parse::<u32>().unwrap();

  let a = d / 365;
  d = d % 365;

  let m = d / 30;
  d = d % 30;

  println!("{} ano(s)
{} mes(es)
{} dia(s)", a, m, d);
}