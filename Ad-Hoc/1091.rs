use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();
  loop {
    input.clear();

    let k: u16 = match stdin.read_line(&mut input) {
      Ok(_) => input.trim()
        .parse()
        .unwrap(),
      Err(_) => break,
    };
  
    if k == 0 {
      break
    }

    let mut input_div = String::new();
    let (m, n): (i32, i32) = match stdin.read_line(&mut input_div) {
      Ok(_) => {
        let values: Vec<i32> = input_div.split_whitespace()
          .map(|x| x.parse().unwrap())
          .collect();
        (values[0], values[1])
      },
      Err(_) => break,
    };
  
    for _ in 0..k {

      let mut input_values = String::new();
  
      let (x, y): (i32, i32) = match stdin.read_line(&mut input_values) {
        Ok(_) => {
          let values: Vec<i32> =input_values.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
          (values[0], values[1])
        },
        Err(_) => break,
      };

      if x == m || y == n {
        println!("divisa");
        continue;
      }
  
      let mut result = String::new();

      if y > n {
        result.push('N');
      } else {
        result.push('S');
      }

      if x > m {
        result.push('E');
      } else {
        result.push('O');
      }

      println!("{}", result);
    }
  }
}