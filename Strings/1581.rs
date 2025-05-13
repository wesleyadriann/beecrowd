use std::io;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    
    let n: u32 = input.trim()
        .parse()
        .unwrap();
        
    for _ in 0..n {
        input.clear();
        
        stdin.read_line(&mut input).unwrap();
        
        let k = input.trim()
            .parse()
            .unwrap();
            
        let mut l: HashSet<String> = HashSet::new();
            
        for _ in 0..k {
            input.clear();
            stdin.read_line(&mut input).unwrap();
            
            let lan = input
                .trim()
                .to_string();
            
            l.insert(lan);
        }
        
        if l.len() > 1 {
            println!("ingles");
        } else {
            let value = l.iter().next().unwrap();
            println!("{}", value);
        }
    }
}