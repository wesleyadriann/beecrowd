use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let n: u32 = match stdin.read_line(&mut input) {
    Ok(_) => input.trim()
      .parse()
      .unwrap(),
    Err(_) => return,
  };

  for _ in 0..n {
    input.clear();

    let (x, y) = match stdin.read_line(&mut input) {
      Ok(_) => {
        let values: Vec<f64> = input.split_whitespace()
          .map(|x| x.parse().unwrap())
          .collect();
        (values[0], values[1])
      },
      Err(_) => break,
    };

    if y == 0.0 {
      println!("divisao impossivel");
      continue;
    }

    println!("{:.1}", x / y);
  }
}