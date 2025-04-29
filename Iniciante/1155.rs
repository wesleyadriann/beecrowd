fn main() {
  let mut result = 1.0;

  for i in 2..=100 {
    result += 1.0/ i as f64;
  }

  println!("{:.2}", result);
}