use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines().skip(1) {
        let line = line.unwrap();
        let line = line.trim();
        let n1 = &line[2..4].parse::<u32>().unwrap();
        let n2 = &line[5..8].parse::<u32>().unwrap();
        let n3 = &line[11..13].parse::<u32>().unwrap();
        
        println!("{}", n1 + n2 + n3);



        
        // let line = line.unwrap();
        // let line_chars = line.chars();
        //
        //
        // let mut prev_is_number = false;
        //
        // let mut result = 0;
        //
        // let mut temp_str = String::with_capacity(12);
        // for char in line_chars {
        //     if !char.is_numeric() {
        //         if prev_is_number {
        //             let n = temp_str.parse::<u32>().unwrap();
        //             result += n;
        //             temp_str.clear();
        //         } 
        //         prev_is_number = false;
        //     } else {
        //         temp_str.push(char);
        //         prev_is_number = true;
        //     }
        // }
        // if !temp_str.is_empty() {
        //     let n = temp_str.parse::<u32>().unwrap();
        //     result += n;
        // }
        // println!("{}", result);
    }
}
