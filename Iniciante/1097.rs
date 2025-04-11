fn main() {
  let mut j_start = 7;
  for i in (1..10).step_by(2) {
    for j in 0..3 {
      println!("I={} J={}", i, j_start - j);
    }
    j_start += 2;
  }
}