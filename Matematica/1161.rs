use std::io;

fn main() {
  let mut input = String::new();

  loop {
    input.clear();
    let values: Vec<u64> = match io::stdin().read_line(&mut input) {
      Ok(_) => {
        if input.trim().len() == 0 {
          break
        }
  
        input.split_whitespace()
          .map(|x| x.parse().unwrap())
          .collect()
      },
      Err(_) => break
    };
  
    let (m, n) = (values[0], values[1]);
  
    let m_fac: u64 = (1..=m).product();
    let n_fac: u64 = (1..=n).product();
  
    println!("{}", m_fac + n_fac);
  }
}