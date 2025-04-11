fn main() {
  let mut i: f64 = 0.0;
  let mut j: f64 = 1.0;

  while i <= 2.0 {
    let round_i = format!("{:.1}", i);
    let formatted_i = if round_i[1..3] == *".0" {
      format!("{}", &round_i[0..1])
    } else {
      round_i
    };

    for plus_j in 0..3 {
      let formatted_j = format!("{:.1}", j + (plus_j as f64));

      if formatted_j[1..3] == *".0" {
        println!("I={} J={}", formatted_i, &formatted_j[0..1]);
      } else {
        println!("I={} J={}", formatted_i, formatted_j);
      }
    }

    j += 0.2;
    i += 0.2;
  }
}