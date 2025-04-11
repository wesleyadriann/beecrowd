fn main() {
  let j_arr = [7,6,5];

  for i in (1..10).step_by(2) {
    for j in j_arr.iter() {
      println!("I={} J={}", i, j);
    }
  }
}