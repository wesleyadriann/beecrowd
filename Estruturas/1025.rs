use std::io;

fn main() {
  let mut case = 1;

  loop {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let values: Vec<u32> = input
      .split_whitespace()
      .map(|x| x.parse().unwrap())
      .collect();

    let n_marbles = values[0];
    let q_search = values[1];

    if n_marbles == 0 && q_search == 0 {
      break;
    }

    let mut marbles = Vec::new();
    for _ in 0..n_marbles {
      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap();
      marbles.push(input.trim().parse::<u32>().unwrap());
    }
    marbles.sort();


    let mut marbles_to_search = Vec::new();
    for _ in 0..q_search {
      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap();
      marbles_to_search.push(input.trim().parse::<u32>().unwrap());
    }

    println!("CASE# {}:", case);
    case += 1;

    for search in marbles_to_search {
      let (found, index) = match marbles.binary_search(&search) {
        Ok(index) => (true, index + 1),
        Err(_) => (false, 0),
      };

      if found {
        let final_index = if marbles[index - 1] == search { index - 1 } else { index };
        println!("{} found at {}", search, final_index);
      } else {
        println!("{} not found", search);
      }
    }

  }
}