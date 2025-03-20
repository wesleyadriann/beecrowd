use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let n: u32 = input.trim()
    .parse()
    .unwrap();

  let mut inside = 0;
  let mut outside = 0;
  for _ in 0..n {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let x: i32 = input.trim()
      .parse()
      .unwrap();

    if x > 9 && x < 21 {
      inside += 1;
    } else {
      outside += 1;
    }
  }

  println!("{} in
{} out", inside, outside);
}