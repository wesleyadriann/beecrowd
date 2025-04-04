use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let n: u32 = match stdin.read_line(&mut input) {
    Ok(_) => input.trim().parse().unwrap(),
    Err(_) => return,
  };

  for i in 0..n {
    let mut input_x = String::new();

    let values: Vec<&str> = match stdin.read_line(&mut input_x) {
      Ok(_) => input_x.split_whitespace().collect(),
      Err(_) => break,
    };

    let (value, base) = (values[0], values[1]);

    println!("Case {}:", i + 1);
    match base {
      "bin" => {
        let from_bin = u32::from_str_radix(&value, 2).unwrap();
        println!("{} dec", from_bin);
        println!("{:x} hex", from_bin);
      },
      "dec" => {
        let from_dec: u32 = value.parse().unwrap();
        println!("{:x} hex", from_dec);
        println!("{:b} bin", from_dec);
      },
      _ => {
        let from_hex = u32::from_str_radix(&value, 16).unwrap();
        println!("{} dec", from_hex);
        println!("{:b} bin", from_hex);
      }
    }
    println!("");
  }
}