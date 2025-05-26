use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();
  let mut lines = stdin.lock()
    .lines();

  let mut city = 0;
  loop {
    let line = lines.next().unwrap().unwrap();
    let n: u32 = line.trim().parse().unwrap();

    if n == 0 {
      break;
    }

    if city > 0 {
      println!();
    }

    let mut total_x = 0;
    let mut total_y = 0;
    let mut consumos = vec![0; 201];

    for _ in 0..n {
      let line = lines.next().unwrap().unwrap();
      let mut inputs = line.split_whitespace();

      let x: i32 = inputs.next()
        .and_then(|a| a.parse().ok())
        .unwrap();
      let y: i32 = inputs.next()
        .and_then(|a| a.parse().ok())
        .unwrap();

      total_x += x;
      total_y += y;
      let index = (y/x) as usize;

      consumos[index] = consumos[index] + x;
    }

    city += 1;
    println!("Cidade# {}:", city);
    
    let mut output = String::with_capacity(1024);
    for i in 0..consumos.len() {
      if consumos[i] > 0 {
        output.push_str(&format!("{}-{} ", consumos[i], i));
      }
    }

    println!("{}", output.trim_end());

    let consumo_total = ((100.0 * total_y as f64) / total_x as f64).floor() / 100.0;

    println!("Consumo medio: {:.2} m3.", consumo_total);
  }
}