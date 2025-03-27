fn main() {
  let mut i = 1;
  let mut j = 60;

  while j >= 0 {
    println!("I={} J={}", i, j);
    i += 3;
    j -= 5;
  }
}