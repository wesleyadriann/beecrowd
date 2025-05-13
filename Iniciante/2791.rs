use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    
    let value: Vec<(usize,u32)> = input.split_whitespace()
        .map(|x| x.parse().unwrap())
        .enumerate()
        .filter(|(_, x)| *x == 1)
        .collect();
        
    println!("{}", value[0].0 + 1);
        
}