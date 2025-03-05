use std::io;

fn main() {
  loop {
    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdin().read_line(&mut input1).unwrap();

    let n = input1.trim().parse::<u32>().unwrap();
    if n == 0 {
      break;
    }

    io::stdin().read_line(&mut input2).unwrap();

    let values: Vec<&str> = input2.split_whitespace().collect();

    let mut x = 0;
    let mut y = 0;

    for value in values {
      let curr = value.parse::<u8>().unwrap();
      if curr == 0 { x += 1} else { y += 1};
    }

    println!("Mary won {} times and John won {} times", x, y);
  } 
}