use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let mut valor = input.trim().parse::<f64>().unwrap();

  let notas_100 = (valor / 100.0).trunc();
  valor = valor % 100.0;

  let notas_50 = (valor / 50.0).trunc();
  valor = valor % 50.0;

  let notas_20 = (valor / 20.0).trunc();
  valor = valor % 20.0;

  let notas_10 = (valor / 10.0).trunc();
  valor = valor % 10.0;

  let notas_5 = (valor / 5.0).trunc();
  valor = valor % 5.0;
  
  let notas_2 = (valor / 2.0).trunc();
  valor = valor % 2.0 * 10.0;

  let moeda_1 = (valor / 10.0).trunc();
  valor = valor % 10.0;

  let moeda_50 = (valor / 5.0).trunc();
  valor = valor % 5.0;

  let moeda_25 = (valor / 2.5).trunc();
  valor = valor % 2.5;

  let moeda_10 = (valor / 1.0).trunc();
  valor = valor % 1.0;

  let moeda_5 = (valor / 0.5).trunc();
  valor = valor % 0.5;

  let moeda_01 = (valor / 0.1).trunc();

  println!("NOTAS:
{} nota(s) de R$ 100.00
{} nota(s) de R$ 50.00
{} nota(s) de R$ 20.00
{} nota(s) de R$ 10.00
{} nota(s) de R$ 5.00
{} nota(s) de R$ 2.00
MOEDAS:
{} moeda(s) de R$ 1.00
{} moeda(s) de R$ 0.50
{} moeda(s) de R$ 0.25
{} moeda(s) de R$ 0.10
{} moeda(s) de R$ 0.05
{} moeda(s) de R$ 0.01", notas_100, notas_50, notas_20, notas_10, notas_5, notas_2,
moeda_1, moeda_50, moeda_25, moeda_10, moeda_5, moeda_01);
}