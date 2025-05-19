use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();
  loop {
    input.clear();
    stdin.read_line(&mut input).unwrap();

    if input[0..1] == *"-" {
      break;
    }

    let value = input.trim();

    if input.starts_with("0x") {
      let num = u32::from_str_radix(&value[2..], 16).unwrap();
      println!("{}", num);
    } else {
      let num: u32 = value.parse().unwrap();
      println!("0x{:X}", num);
    }
  }
}

