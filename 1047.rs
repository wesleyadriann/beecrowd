use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let values: Vec<&str> = input.split_whitespace().collect();

  let h1 = values[0].parse::<i16>().unwrap();
  let m1 = values[1].parse::<i16>().unwrap();

  let h2 = values[2].parse::<i16>().unwrap();
  let m2 = values[3].parse::<i16>().unwrap();

  let t1 = h1 * 60 + m1;
  let t2 = h2 * 60 + m2;

  let t = if t2 > t1 { t2 - t1 } else { t1 - t2 + 24 * 60 };

  let hours = t / 60;
  let minutes = t % 60;

  println!("O JOGO DUROU {} HORA(S) E {} MINUTO(S)", hours, minutes);
}