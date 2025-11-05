use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let ddd = input.trim().parse::<u16>().unwrap();

  let x = match ddd {
    61 => "Brasilia",
    71 => "Salvador",
    11 => "Sao Paulo",
    21 => "Rio de Janeiro",
    32 => "Juiz de Fora",
    19 => "Campinas",
    27 => "Vitoria",
    31 => "Belo Horizonte",
    _ => "DDD nao cadastrado",
  };

  println!("{}", x);
}