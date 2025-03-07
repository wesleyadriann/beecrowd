use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let values: Vec<&str> = input.split_whitespace().collect();

  let x = values[0].parse::<i32>().unwrap();
  let y = values[1].parse::<i32>().unwrap();
  let z = values[2].parse::<i32>().unwrap();

  let mut result = [x, y, z];

  if result[1] < result[0] {
    result[0] = y;
    result[1] = x;
  }

  if result[2] < result[1] {
    let curr = result[1];
    result[1] = result[2];
    result[2] = curr;
  }

  if result[1] < result[0] {
    let curr = result[0];
    result[0] = result[1];
    result[1] = curr;
  }

  println!("{}\n{}\n{}\n\n{}\n{}\n{}", result[0], result[1], result[2], x, y, z);

}