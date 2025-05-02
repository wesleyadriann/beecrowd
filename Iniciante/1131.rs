use std::io;

fn main() {
  let stdin = io::stdin();

  let mut inter = 0;
  let mut gremio = 0;
  let mut empate = 0;

  loop {
    let mut input = String::new();

    let gols: Vec<u32> = match stdin.read_line(&mut input) {
      Ok(_) => input.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect(),
      Err(_) => continue
    };

    if gols[0] > gols[1] {
      inter += 1;
    } else if gols[1] > gols[0] {
      gremio += 1;
    } else {
      empate += 1;
    }

    input.clear();
    println!("Novo grenal (1-sim 2-nao)");
    let again: u32 = match stdin.read_line(&mut input) {
      Ok(_) => input.trim()
        .parse()
        .unwrap(),
      Err(_) => continue,
    };

    if again == 2 {
      break;
    }
  }

  let total = inter + gremio + empate;
  println!("{} grenais", total);
  println!("Inter:{}", inter);
  println!("Gremio:{}", gremio);
  println!("Empates:{}", empate);

  if inter == gremio {
    println!("Nao houve vencedor");
  } else if inter > gremio {
    println!("Inter venceu mais");
  } else {
    println!("Gremio venceu mais");
  }
}