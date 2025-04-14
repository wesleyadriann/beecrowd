use std::io;

fn main(){
  let stdin = io::stdin();

  let mut input = String::new();

  let n: u16 = match stdin.read_line(&mut input) {
    Ok(_) => input.trim()
      .parse()
      .unwrap(),
    Err(_) => return,
  };

  let mut result: (String, u32, u32, u32) = (String::new(), 0, 0, 0);
  for _ in 0..n {
    let mut input_values = String::new();
    stdin.read_line(&mut input_values).unwrap();
  
    {
      let values: Vec<&str> = input_values.split_whitespace()
        .collect();
  
      let (s, p, k, m) = (
        values[0].to_string(),
        values[1].parse::<u32>().unwrap(),
        values[2].parse::<u32>().unwrap(),
        values[3].parse::<u32>().unwrap(),
      );
  
      if p < result.1 {
        continue;
      }

      if p > result.1 {
        result = (s, p, k, m);
        continue;
      }

      if k < result.2 {
        continue;
      }

      if k > result.2 {
        result = (s, p, k, m);
        continue;
      }

      if m > result.3 {
        continue;
      }

      if m < result.3 {
        result = (s, p, k, m);
      } else if  s < result.0 {
        result = (s, p, k, m);
      } 
    }
  }

  println!("{}",result.0);
}