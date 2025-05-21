use std::io::{
  self,
  BufRead,
  BufReader
};

fn main() {
  let mut input = String::new();
  let mut reader = BufReader::new(io::stdin());

  reader.read_line(&mut input).unwrap();

  let c: u32 = input.trim()
    .parse()
    .unwrap();
  
  for _ in 0..c {
    input.clear();
    reader.read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let n: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();
  
    let notes = values.map(|x| x.parse::<f64>().unwrap());
    let median = notes.clone().sum::<f64>() / n as f64;

    let mut qtd_above: u32 = 0;
    
    for note in notes {
      if note > median {
        qtd_above += 1;
      }
    }

    println!("{:.3}%", (qtd_above as f64 / n as f64) * 100.0);
  }
}