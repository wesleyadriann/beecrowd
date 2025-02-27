use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let vec: Vec<&str> = input.split_whitespace().collect();

  let a = vec[0].parse::<i32>().unwrap();
  let b = vec[1].parse::<i32>().unwrap();
  let c = vec[2].parse::<i32>().unwrap();
  let d = vec[3].parse::<i32>().unwrap();

  let b_gt_c = b > c;
  let d_gt_a = d > a;
  let cd_gt_ab = (c + d) > (a + b);
  let cd_is_n = c > -1 && d > -1;
  let a_is_odd = a % 2 == 0;

  if b_gt_c && d_gt_a && cd_gt_ab && cd_is_n && a_is_odd {
    println!("Valores aceitos");
  } else {
    println!("Valores nao aceitos");
  }
}