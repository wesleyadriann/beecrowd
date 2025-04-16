use std::io;
use std::cmp::Ordering;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();
  let mut i = 0;
  loop {
    input.clear();

    let characters: Vec<char> = match stdin.read_line(&mut input) {
      Ok(_) => {
        let result = input.trim();
        if result.len() == 0 {
          break;
        }
        
        result
          .chars()
          .collect()
      },
      Err(_) => return,
    };

    if i > 0 {
      println!("");
    } else {
      i += 1;
    }

    let mut result: Vec<(usize, u32)> = vec![(0,0); 128];

    for cha in characters {
      let idx = cha as usize;
      result[idx] = (idx, result[idx].1 + 1);
    }

    result.sort_by(|(asc_a, a), (asc_b, b)| {
      match a.cmp(b) {
        Ordering::Equal => asc_b.cmp(asc_a),
        other => other,
    }
    });
    result.retain(|&(_, x)| x > 0);

    for (asc_value, qtd) in result {
      println!("{} {}", asc_value, qtd);
    }
  }
}