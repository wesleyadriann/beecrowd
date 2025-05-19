use std::io;

fn main() {
  let mut input_number = String::new();
  let mut input_hours = String::new();
  let mut input_salary = String::new();

  io::stdin().read_line(&mut input_number).unwrap();
  io::stdin().read_line(&mut input_hours).unwrap();
  io::stdin().read_line(&mut input_salary).unwrap();

  let number = input_number.trim().parse::<u32>().unwrap();
  let hours = input_hours.trim().parse::<f64>().unwrap();
  let salary = input_salary.trim().parse::<f64>().unwrap();

  println!("NUMBER = {}", number);
  println!("SALARY = U$ {:.2}", (salary * hours));
}