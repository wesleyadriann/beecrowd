use std::io;

fn main() {
  let mut input_n = String::new();

  io::stdin().read_line(&mut input_n).unwrap();

  let n = input_n.trim().parse::<u32>().unwrap();

  for _ in 0..n {
    let mut input_number = String::new();

    io::stdin().read_line(&mut input_number).unwrap();

    let numbers: Vec<char> = input_number.trim().chars().collect();

    let mut qtd = 0;

    for number in numbers {
      let qtd_for_number = match number {
        '0' => 6,
        '1' => 2,
        '2' => 5,
        '3' => 5,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 3,
        '8' => 7,
        '9' => 6,
        _ => 0,
      };
      qtd += qtd_for_number;
    }

    println!("{} leds", qtd);
  }
}