use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();

  let mut lines = stdin.lock()
    .lines()
    .map(|x| x.unwrap());

  loop {
    let line = lines.next().unwrap();
    let n: u16 = line.parse().unwrap();

    if n == 0 {
      break;
    }

    for _ in 0..n {
      let line = lines.next().unwrap();

      let responses: Vec<u16> = line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

      let is_a = responses[0] < 128;
      let is_b = responses[1] < 128;
      let is_c = responses[2] < 128;
      let is_d = responses[3] < 128;
      let is_e = responses[4] < 128;

      if is_a && !is_b && !is_c && !is_d && !is_e {
        println!("A");
      } else if is_b && !is_a && !is_c && !is_d && !is_e {
        println!("B");
      } else if is_c && !is_a && !is_b && !is_d && !is_e {
        println!("C");
      } else if is_d && !is_a && !is_b && !is_c && !is_e {
        println!("D");
      } else if is_e && !is_a && !is_b && !is_c && !is_d {
        println!("E");
      } else {
        println!("*");
      }
    }
  }

}
    