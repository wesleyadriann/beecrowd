use std::io;

fn main() {
  let mut input_d1 = String::new();
  let mut input_h1 = String::new();

  let mut input_d2 = String::new();
  let mut input_h2 = String::new();

  io::stdin().read_line(&mut input_d1).unwrap();
  io::stdin().read_line(&mut input_h1).unwrap();

  io::stdin().read_line(&mut input_d2).unwrap();
  io::stdin().read_line(&mut input_h2).unwrap();

  let d1 = input_d1
    .split_whitespace()
    .last()
    .unwrap();

  let d2 = input_d2
    .split_whitespace()
    .last()
    .unwrap();

  let d1: u32 = d1.parse().unwrap();
  let d2: u32 = d2.parse().unwrap();

  let hms1: Vec<u32> = input_h1
    .split(':')
    .map(|x| x.trim().parse().unwrap())
    .collect();

  let hms2: Vec<u32> = input_h2
    .split(':')
    .map(|x| x.trim().parse().unwrap())
    .collect();
  
  let t1 = (d1 * 86400) + (hms1[0] * 3600) + (hms1[1] * 60) + hms1[2];
  let t2 = (d2 * 86400) + (hms2[0] * 3600) + (hms2[1] * 60) + hms2[2];

  let t = t2 - t1;

  let days = t / 86400;
  let hours = (t % 86400) / 3600;
  let minutes = (t % 86400 % 3600) / 60;
  let seconds = (t % 86400 % 3600) % 60;

  println!("{} dia(s)
{} hora(s)
{} minuto(s)
{} segundo(s)", days, hours, minutes, seconds);
}