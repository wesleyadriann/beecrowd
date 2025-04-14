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

    let t: i64 = match stdin.read_line(&mut input) {
      Ok(_) => input.trim()
        .parse()
        .unwrap(),
      Err(_) => return,
    };

    let result = 2015 - t;
    if result > 0 {
      println!("{} D.C.", result);
    } else {
      println!("{} A.C.", result.abs() + 1);
    }
    
  }
}