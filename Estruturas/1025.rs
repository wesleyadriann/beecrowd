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
      let (found, index) = pesquisa_binaria(&marbles, search);

      if found {
        println!("{} found at {}", search, index);
      } else {
        println!("{} not found", search);
      }
    }

  }
}

fn pesquisa_binaria(arr: &Vec<u32>, valor: u32) -> (bool, usize) {
  let arr_len = arr.len();

  let mut inicio = 0;
  let mut fim = arr_len;

  while inicio < fim {
    let meio = (inicio + fim) / 2;
    if arr[meio] < valor {
      inicio = meio + 1;
    }
    else {
      fim = meio;
    }
  }

  if inicio >= arr_len {
    return (false, 0);
  }

  if arr[inicio] == valor {
    (true, inicio + 1)
  } else {
    (false, 0)
  }
}