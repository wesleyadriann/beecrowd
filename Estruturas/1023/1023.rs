use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();
  let mut lines = stdin.lock()
    .lines()
    .map(|x| x.unwrap());

  let mut city = 0;
  loop {
    let line = lines.next().unwrap();
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
      let line = lines.next().unwrap();
      let inputs: Vec<i32> = line.split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
      let (x, y) = (inputs[0], inputs[1]);
      total_x += x;
      total_y += y;
      let index = (y/x) as usize;

      consumos[index] = consumos[index] + x;
    }

    city += 1;
    println!("Cidade# {}:", city);
    
    let mut output = Vec::new();
    for i in 0..consumos.len() {
      if consumos[i] > 0 {
        output.push(format!("{}-{}", consumos[i], i));
      }
    }

    println!("{}", output.join(" "));

    let consumo_total = ((100.0 * total_y as f64) / total_x as f64).floor() / 100.0;

    println!("Consumo medio: {:.2} m3.", consumo_total);
  }
}