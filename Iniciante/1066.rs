use std::io;

fn main() {
  let mut input = String::new();

  for _ in 0..5 {
    io::stdin().read_line(&mut input).unwrap();
  }

  let lines: Vec<i32> = input.lines()
    .map(|x| x.trim().parse().unwrap())
    .collect();

  let mut par = 0;
  let mut imp = 0;
  let mut pos = 0;
  let mut neg = 0;

  for line in lines {
    if line % 2 == 0 {
      par += 1;
    } else {
      imp += 1;
    }

    if line > 0 {
      pos += 1;
    } else if line < 0 {
      neg += 1;
    }
  }

  println!("{} valor(es) par(es)
{} valor(es) impar(es)
{} valor(es) positivo(s)
{} valor(es) negativo(s)", par, imp, pos, neg);

}