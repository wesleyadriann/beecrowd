use std::io;

fn main() {
  let mut input_1 = String::new();
  let mut input_2 = String::new();

  io::stdin().read_line(&mut input_1).unwrap();
  io::stdin().read_line(&mut input_2).unwrap();

  let item_1: Vec<&str> = input_1.split_whitespace().collect();
  let item_2: Vec<&str> = input_2.split_whitespace().collect();

  let item_1_qtd = item_1[1].parse::<f64>().unwrap();
  let item_2_value = item_1[2].parse::<f64>().unwrap();

  let item_2_qtd = item_2[1].parse::<f64>().unwrap();
  let item2_value = item_2[2].parse::<f64>().unwrap();

  println!("VALOR A PAGAR: R$ {:.2}", item_1_qtd * item_2_value + item_2_qtd * item2_value);
}