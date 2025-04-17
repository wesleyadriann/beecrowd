use std::io;

fn main() {
  let stdin = io::stdin();

  loop {
    let mut input = String::new();

    let (m, n) = match stdin.read_line(&mut input) {
      Ok(_) => {
        let temp: Vec<i32> = input.split_whitespace()
          .map(|x| x.parse().unwrap())
          .collect();

        if input.trim().len() == 0 ||
          temp[0] < 1 ||
          temp[1] < 1 {
          break;
        }

        (temp[0], temp[1])
      },
      Err(_) => break,
    };

    let (start, end) = if m > n { (n, m) } else { (m,n) };

    let mut result = 0;

    for i in start..=end {
      print!("{} ", i);
      result += i;
    }
    print!("Sum={}\n", result);
  }
}