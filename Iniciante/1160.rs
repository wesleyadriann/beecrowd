use std::io::{
  self,
  BufRead,
  BufReader
};

fn main() {
  let mut reader = BufReader::new(io::stdin());

  let mut input = String::with_capacity(28);

  reader.read_line(&mut input).unwrap();

  let t: u32 = input.trim()
    .parse()
    .unwrap();

  for _ in 0..t {
    input.clear();

    reader.read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let mut pa: f64 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let mut pb: f64 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let g1 = values.next()
      .and_then(|x| x.parse::<f64>().ok())
      .unwrap() / 100.0;

    let g2 = values.next()
      .and_then(|x| x.parse::<f64>().ok())
      .unwrap() / 100.0;

    let mut t = 0;
    while pa <= pb && t <= 100 {
      pa += (pa * g1).floor();
      pb += (pb * g2).floor();
      t += 1;
    }

    if t > 100 {
      println!("Mais de 1 seculo.");
    } else {
      println!("{:.0} anos.", t);
    }
  }
}

