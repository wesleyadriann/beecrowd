use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let valor = input.trim().parse::<u32>().unwrap();

  let mut remainder = valor;

  let notas_100 = remainder / 100;
  remainder = remainder % 100;

  let notas_50 = remainder / 50;
  remainder = remainder % 50;

  let notas_20 = remainder / 20;
  remainder = remainder % 20;

  let notas_10 = remainder / 10;
  remainder = remainder % 10;

  let notas_5 = remainder / 5;
  remainder = remainder % 5;

  let notas_2 = remainder / 2;
  remainder = remainder % 2;

  let notas_1 = remainder / 1;

  println!("{}
{} nota(s) de R$ 100,00
{} nota(s) de R$ 50,00
{} nota(s) de R$ 20,00
{} nota(s) de R$ 10,00
{} nota(s) de R$ 5,00
{} nota(s) de R$ 2,00
{} nota(s) de R$ 1,00", valor, notas_100, notas_50, notas_20, notas_10, notas_5, notas_2, notas_1);
}