use std::io;
use std::collections::HashMap;
use std::cmp;

// WIP
fn main() {
  loop {
    let mut input_n = String::new();
  
    io::stdin().read_line(&mut input_n).unwrap();
  
    let n = input_n.trim().parse::<usize>().unwrap(); 

    if n == 0 {
      break;
    }
    
    let mut input_strings = String::new();
    for _ in 0..n {
      io::stdin().read_line(&mut input_strings).unwrap();
    }
    
    let mut strings: Vec<&str> = input_strings.trim().lines().collect();

    strings.sort();
    let mut longest = HashMap::new();
    let mut max_length = 1;

    for s in strings {
      println!("s {}", s);
      longest.insert(s,1);

      let string_len = s.len() ;

      for i in 1..string_len {
        let (s_left, s_right) = s.split_at(i);

        let s_left_value = match longest.get(s_left) {
          Some(&number) => number,
          _ => 0,
        };

        let s_right_value = match longest.get(s_right) {
          Some(&number) => number,
          _ => 0,
        };

        if s_left_value > 0 {
          let s1 = longest.get(s).unwrap();
          longest.insert(s, cmp::max(*s1, s_left_value + 1));
        }

        if s_right_value > 0 {
          let s1 = longest.get(s).unwrap();
          longest.insert(s, cmp::max(*s1, s_right_value + 1));
        }
      }

      let curr = longest.get(s).unwrap();
      max_length = cmp::max(max_length, *curr);
    }


    println!("{}", max_length);
  }
}