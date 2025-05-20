use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::with_capacity(32);
  loop {
    input.clear();

    stdin.read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let ev1 = values.next()
      .and_then(|x| x.parse::<f32>().ok())
      .unwrap();

    let ev2 = values.next()
      .and_then(|x| x.parse::<f32>().ok())
      .unwrap();

    let at = values.next()
      .and_then(|x| x.parse::<f32>().ok())
      .unwrap();

    let d = values.next()
      .and_then(|x| x.parse::<f32>().ok())
      .unwrap();

    if ev1 == 0.0 && ev2 == 0.0 && at == 0.0 && d == 0.0 {
      break
    }
    let v1 = ((ev1 + d - 1.0) / d).floor();
    let v2 = ((ev2 + d - 1.0) / d).floor();

    if at == 3.0 {
      let result = v1 / (v1 + v2);
      println!("{:.1}", result * 100.0);
    } else {
      let v1_hit = 1.0 - (6.0 - at) / 6.0;
      let v1_fail = (1.0 - v1_hit) / v1_hit;
      let result = (1.0 - v1_fail.powf(v1)) / (1.0 - v1_fail.powf(v1 + v2));
      println!("{:.1}", result * 100.0);
    }
  }
}
