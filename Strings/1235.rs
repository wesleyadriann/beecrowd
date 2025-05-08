use std::io;

fn main() {
    let stdin = io::stdin();
    
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    
    let n: u32 = input.trim()
        .parse()
        .unwrap_or(0);
        
    for _ in 0..n {
        input.clear();
        
        stdin.read_line(&mut input).unwrap();
        
        let word = input.replace("\r\n", "").replace("\n", "");
        
        let str_len = word.len();
        let str_mid = str_len / 2;
    
        let start: String = (&word[..str_mid]).chars()
            .rev()
            .collect();
            
        let end: String = (&word[str_mid..]).chars()
            .rev()
            .collect();
        
        println!("{}{}", start, end);
    }
}