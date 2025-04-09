use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let n: u16 = match stdin.read_line(&mut input) {
    Ok(_) => input.trim()
      .parse()
      .unwrap(),
    Err(_) => return,
  };

  let mut numbers: [u16; 2001] = [0; 2001];
  for _ in 0..n {
    let mut input_x = String::new();

    let x: usize = match stdin.read_line(&mut input_x) {
      Ok(_) => input_x.trim()
        .parse()
        .unwrap(),
      Err(_) => break,
    };

    numbers[x] = numbers[x] + 1;
  }

  for (i, value) in numbers.iter_mut().enumerate() {
    if *value > 0 {
      println!("{} aparece {} vez(es)", i, value);
    }
  }
}