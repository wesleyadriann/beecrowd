use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let n = match stdin.read_line(&mut input) {
    Ok(_) => input.trim()
      .parse()
      .unwrap(),
    Err(_) => return,
  };

  let mut total = 0.0;
  let mut total_c = 0.0;
  let mut total_r = 0.0;
  let mut total_s = 0.0;

  for _ in 0..n {
    let mut input_t = String::new();

    let result: Vec<&str> = match stdin.read_line(&mut input_t) {
      Ok(_) => input_t.split_whitespace()
        .collect(),
      Err(_) => break,
    };

    let (q, t): (f64, &str) = (
      result[0].parse().unwrap(),
      result[1]
    );

    total += q;
    match t {
      "C" => total_c += q,
      "R" => total_r += q,
      _ => total_s += q,
    }
  }

  println!("Total: {} cobaias", total);
  println!("Total de coelhos: {}", total_c);
  println!("Total de ratos: {}", total_r);
  println!("Total de sapos: {}", total_s);
  println!("Percentual de coelhos: {:.2} %", total_c / total * 100.0);
  println!("Percentual de ratos: {:.2} %", total_r / total * 100.0);
  println!("Percentual de sapos: {:.2} %", total_s / total * 100.0);
}