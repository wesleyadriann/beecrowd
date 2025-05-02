use std::io;

fn main() {
  let stdin = io::stdin();
  
  'global_loop: loop {
    let mut input = String::new();

    let mut notes: Vec<f64> = Vec::new();

    while notes.len() < 2 {
      input.clear();
      let note: f64 = match stdin.read_line(&mut input) {
        Ok(_) => input.trim()
          .parse()
          .unwrap(),
        Err(_) => continue,
      };

      if note < 0.0 || note > 10.0 {
        println!("nota invalida");
      } else {
        notes.push(note)
      }
    }

    let result: f64 = (notes[0] + notes[1]) / 2.0;
    println!("media = {:.2}", result);

    loop {
      println!("novo calculo (1-sim 2-nao)");
      input.clear();
  
      let again: i16 = match stdin.read_line(&mut input) {
        Ok(_) => input.trim()
          .parse()
          .unwrap(),
        Err(_) => break
      };
  
      match again {
        1 => break,
        2 => break 'global_loop,
        _ => continue
      };
    }
  }
}