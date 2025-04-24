use std::io;

fn main() {
  let stdin = io::stdin();

  loop {
    let mut input = String::new();

    let (x, y) = match stdin.read_line(&mut input) {
      Ok(_) => {
        let values: Vec<i32> = input.split_whitespace()
          .map(|x| x.parse().unwrap())
          .collect();
        (values[0], values[1])
      },
      Err(_) => break,
    };

    if x == 0 || y == 0 {
      break;
    }

    if x > 0 && y > 0 {
      println!("primeiro"); 
    } else if x < 0 && y > 0 {
      println!("segundo");
    } else if x < 0 && y < 0 {
      println!("terceiro");
    } else {
      println!("quarto");
    }
  }
}