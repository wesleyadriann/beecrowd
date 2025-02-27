use std::io;

fn main() {
  let mut input_name = String::new();
  let mut input_salary = String::new();
  let mut input_sales = String::new();

  io::stdin().read_line(&mut input_name).unwrap();
  io::stdin().read_line(&mut input_salary).unwrap();
  io::stdin().read_line(&mut input_sales).unwrap();

  let salary = input_salary.trim().parse::<f64>().unwrap();
  let sales = input_sales.trim().parse::<f64>().unwrap();

  println!("TOTAL = R$ {:.2}", (sales * 0.15) + salary);
}