fn main() {
  let mut result = 0.0;
  for i in 1..=20 {
    let num = (2 * i - 1) as f64;

    let pow_i = (i - 1) as f64;
    let den = 2f64.powf(pow_i);
    
    result += num/den;
  }

  println!("{:.2}", result);
}