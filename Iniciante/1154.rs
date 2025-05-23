use std::io::{
  self,
  BufRead,
  BufReader
};

fn main() {
  let mut reader = BufReader::new(io::stdin());
  let mut input = String::with_capacity(8);

  let mut total: u32 = 0;
  let mut qtd: u32 = 0;

  loop {
    input.clear();
    reader.read_line(&mut input).unwrap();

    if &input[0..1] == "-" {
      break
    }

    let num: u32 = input.trim()
      .parse()
      .unwrap();

    total += num;
    qtd += 1;
  }

  println!("{:.2}", total as f64 /qtd as f64);
}