use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    
    let vogais = ['a', 'e', 'i', 'o', 'u'];
    
    let chars = input.trim()
        .chars()
        .filter(|x| vogais.contains(&x));
    
    let word: String = chars.clone()
        .collect();
        
    let rev_word: String = chars.clone()
        .rev()
        .collect();
        
    if word == rev_word {
        println!("S");
    } else {
        println!("N");
    }
}