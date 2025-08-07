use std::io::{self, BufRead, BufReader};

fn main() {
    let mut reader = BufReader::new(io::stdin());

    let mut input = String::with_capacity(8);

    let mut odd_n: Vec<i32> = Vec::with_capacity(5);
    let mut even_n: Vec<i32> = Vec::with_capacity(5);

    for _ in 0..15 {
        input.clear();

        reader.read_line(&mut input).unwrap();

        let value = input.trim().parse::<i32>().unwrap();

        let is_odd = value % 2 == 0;

        if is_odd {
            odd_n.push(value);
        } else {
            even_n.push(value);
        }

        if odd_n.len() == 5 {
            print_vec(&mut odd_n, "par");
        }

        if even_n.len() == 5 {
            print_vec(&mut even_n, "impar");
        }
    }

    print_vec(&mut even_n, "impar");
    print_vec(&mut odd_n, "par");
}
 
fn print_vec(vec: &mut Vec<i32>, label: &str) {
   let mut values = vec.iter().enumerate();
    while let Some(value) = values.next() {
        let (i, &value) = value;
        println!("{}[{}] = {}", label, i, value);
    } 
    vec.clear();
}
