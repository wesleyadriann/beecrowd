use std::io;
// wip
fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let values: Vec<f64> = match stdin.read_line(&mut input) {
    Ok(_) => input.split_whitespace()
      .map(|x| x.parse().unwrap())
      .collect()
    ,
    Err(_) => return,
  };

  input.clear();
  let names: Vec<&str> = match stdin.read_line(&mut input) {
    Ok(_) => input.split_whitespace().collect(),
    Err(_) => return,
  };

  let n = values[0] as usize;
  let (l, q) = (values[1], values[2]);
  
  let total_c = (l / q).floor() as usize;
  let idx = (total_c) % n;

  let rem = l.rem_euclid(q);
  let last_rem = if rem == 0.0 { q } else { rem };

  println!("{} {:.1}", names[idx - 1], last_rem);
}