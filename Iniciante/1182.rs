use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  stdin.read_line(&mut input).unwrap();

  let mut next_idx: u16 = input.trim()
    .parse()
    .unwrap();

  input.clear();

  stdin.read_line(&mut input).unwrap();

  let t = input.trim();

  let mut input_items = String::new();

  let mut sum = 0.0;
  for i in 0..144 {
    input_items.clear();
    stdin.read_line(&mut input_items).unwrap();
    if i == next_idx {
      sum += input_items.trim()
        .parse::<f64>()
        .unwrap();
      next_idx += 12;
    }
  }

  if t == "S" {
    println!("{:.1}", sum);
  } else {
    println!("{:.1}", sum / 12.0);
  }
}