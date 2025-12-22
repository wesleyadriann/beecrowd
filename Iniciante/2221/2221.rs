use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();

    let t = lines.next()
        .unwrap()
        .and_then(|x| Ok(x.parse::<u32>().unwrap()))
        .unwrap();

    for _ in 0..t {
        let b = lines.next()
            .unwrap()
            .and_then(|x| Ok(x.parse::<f64>().unwrap()))
            .unwrap();

        let line = lines.next()
            .unwrap()
            .unwrap();

        let mut line_values = line 
            .split_whitespace();

        let a1 = line_values.next()
            .and_then(|x| x.parse::<u32>().ok())
            .unwrap();

        let d1 = line_values.next()
            .and_then(|x| x.parse::<u32>().ok())
            .unwrap();

        let l1 = line_values.next()
            .and_then(|x| x.parse::<u32>().ok())
            .unwrap();

        let line = lines.next()
            .unwrap()
            .unwrap();

        let mut line_values = line 
            .split_whitespace();

        let a2 = line_values.next()
            .and_then(|x| x.parse::<u32>().ok())
            .unwrap();

        let d2 = line_values.next()
            .and_then(|x| x.parse::<u32>().ok())
            .unwrap();

        let l2 = line_values.next()
            .and_then(|x| x.parse::<u32>().ok())
            .unwrap();
 

        let mut vp1 = (a1 + d1) as f64 / 2.0;
        if l1 % 2 == 0 {
            vp1 += b as f64;
        }

        let mut vp2 = (a2 + d2) as f64 / 2.0;
        if l2 % 2 == 0 {
            vp2 += b as f64;
        }

        if vp1 == vp2 {
            println!("Empate");
        } else if vp1 > vp2 {
            println!("Dabriel");
        } else {
            println!("Guarte");
        }
    }
}
