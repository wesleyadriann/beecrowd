use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();
  stdin.read_line(&mut input).unwrap();

  let reg = to_binary(input.trim());

  let power_on_bit = reg.chars()
    .nth_back(0)
    .unwrap() as u8 - 48;
  let power_on = check_status(power_on_bit);

  let error_bit = reg.chars()
    .nth_back(1)
    .unwrap() as u8 - 48;
  let error = check_status(error_bit);

  let ready_bit = reg.chars()
    .nth_back(2)
    .unwrap() as u8 - 48;
  let ready = check_status(ready_bit);

  let overheat_bit = reg.chars()
    .nth_back(3)
    .unwrap() as u8 - 48;
  let overheat = check_status(overheat_bit); 

  input.clear();
  stdin.read_line(&mut input).unwrap();

  println!("===========================");
  println!(" REGISTER STATE");
  println!("===========================");
  println!("  Power On    : {}", power_on);
  println!("  Error       : {}", error);
  println!("  Ready       : {}", ready);
  println!("  Overheat    : {}", overheat);
  println!("  Mode ID     : {}", input.trim());
}


fn to_binary(input: &str) -> String {
  let num = if input.starts_with("0b") {
    u32::from_str_radix(&input[2..], 2)
  } else if input.starts_with("0x") {
    u32::from_str_radix(&input[2..], 16)
  } else {
    input.parse::<u32>()
  };
  format!("{:032b}", num.unwrap())
}

fn check_status(input: u8) -> &'static str {
  if input == 0 { "No" } else { "Yes" }
}