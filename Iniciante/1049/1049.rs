use std::io;

fn main() {
  let mut input1 = String::new();
  let mut input2 = String::new();
  let mut input3 = String::new();

  io::stdin().read_line(&mut input1).unwrap();
  io::stdin().read_line(&mut input2).unwrap();
  io::stdin().read_line(&mut input3).unwrap();

  input1 = input1.trim().to_string();
  input2 = input2.trim().to_string();
  input3 = input3.trim().to_string();

  let mut result = String::new();
  if input1 == "vertebrado" {
    if input2 == "ave" {
      if input3 == "carnivoro" {
        result.push_str("aguia");
      } else {
        result.push_str("pomba");
      }
    } else {
      if input3 == "onivoro" {
        result.push_str("homem");
      } else {
        result.push_str("vaca");
      }
    }
  } else {
    if input2 == "inseto" {
      if input3 == "hematofago" {
        result.push_str("pulga");
      } else {
        result.push_str("lagarta");
      }
    } else {
      if input3 == "hematofago" {
        result.push_str("sanguessuga");
      } else {
        result.push_str("minhoca");
      }
    }
  }
  println!("{}",result);
}