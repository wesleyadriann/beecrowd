use std::io;

fn main() {
  let stdin = io::stdin();
  
  let mut input = String::new();

  stdin.read_line(&mut input).unwrap();

  let n: u32 = input.trim()
    .parse()
    .unwrap();

  let mut even: Vec<u32> = Vec::new();
  let mut odd: Vec<u32> = Vec::new();

  for _ in 0..n {
    let mut input_number = String::new();

    let number: u32 = match stdin.read_line(&mut input_number) {
      Ok(_) => input_number.trim().parse().unwrap(),
      Err(_) => break,
    };

    if number % 2 == 0 {
      even.push(number);
    } else {
      odd.push(number);
    }
  }

  even.sort();
  odd.sort_by(|a , b| b.cmp(a));

  let even_result = even.iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>()
    .join("\n");

  let odd_result = odd.iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>()
    .join("\n");

  println!("{}", even_result);
  println!("{}", odd_result);
  
}