use std::io;
use std::collections::BTreeMap;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();
  let n: u32 = match stdin.read_line(&mut input) {
    Ok(_) => input.trim().parse().unwrap(),
    Err(_) => return
  };

  let mut pecas: BTreeMap<String, u32> = BTreeMap::new();
  let mut pecas_compostas: BTreeMap<String, Vec<(String, u32)>> = BTreeMap::new();

  for _ in 0..n {
    let mut peca_input = String::new();

    stdin.read_line(&mut peca_input).unwrap();

    let peca: Vec<&str> = peca_input.split_whitespace().collect();

    let peca_nome = peca[0].to_string();
    let peca_valor: u32 = peca[1].trim()
      .parse()
      .unwrap();

    pecas.insert(peca_nome, peca_valor);
  }

  loop {
    let mut line = String::new();

    match stdin.read_line(&mut line) {
      Ok(_) => {
        if line.trim().len() == 0 {
          break
        }
        let infos: Vec<&str> = line.split_whitespace().collect();

        let p1 = infos[0].to_string();
        let p2 = infos[1].to_string();
        let p2_qtd: u32 = infos[2].trim().parse().unwrap();

        pecas_compostas.entry(p1).or_insert_with(Vec::new).push((p2, p2_qtd))
      
      },
      Err(_) => break
    }
  }


  let mut result: BTreeMap<String, u32> = BTreeMap::new();

  for (key, _) in pecas_compostas.iter() {
    find_peca_value(&key, &pecas_compostas, &mut result, &mut pecas);
  }

  for (key, val) in result.iter() {
    println!("{} {}", key, val);
  }
}

fn find_peca_value(
  peca_to_find: &String,
  pecas_compostas: &BTreeMap<String, Vec<(String, u32)>>,
  result: &mut BTreeMap<String, u32>,
  pecas: &mut BTreeMap<String, u32>,
) -> u32 {

  if pecas.contains_key(peca_to_find) {
    return *pecas.get(peca_to_find).unwrap();
  }
  let mut total: u32 = 0;

  let peca = pecas_compostas.get(peca_to_find).unwrap();

  for (sub_peca, qtd) in peca {
    if pecas.contains_key(sub_peca) {
      total += *pecas.get(sub_peca).unwrap() * qtd;
    } else {
      total += find_peca_value(sub_peca, pecas_compostas, result, pecas) * qtd;
    }
  }

  pecas.insert(peca_to_find.to_string(), total);
  result.insert(peca_to_find.to_string(), total);
  total
}