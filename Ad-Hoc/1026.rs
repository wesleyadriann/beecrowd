use std::io;

fn main() {

  loop {
    let mut input = String::new();
  
    io::stdin().read_line(&mut input).unwrap();
  
    let values: Vec<u32> = input.split_whitespace()
      .map(|x| x.parse().unwrap())
      .collect();
  
    if values.len() == 0 {
      break;
    }

    let (x, y) = (values[0], values[1]);
  
    println!("{}", x ^ y);

    // manual bit xor
    // let mut result: u32 = 0;
    // for i in 0..32 {
    //   let x_bit = (x >> i) & 1;
    //   let y_bit = (y >> i) & 1;
    //   if x_bit != y_bit {
    //     result = result | (1 << i);
    //   }
    // }
  }
}