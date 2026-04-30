use std::{io, cmp};

fn main() {
    let stdin = io::stdin();

    let mut input_s = String::with_capacity(10000);
    let mut input_t = String::with_capacity(10000);

    stdin.read_line(&mut input_s).unwrap();
    stdin.read_line(&mut input_t).unwrap();

    let mut s = input_s.trim().chars();
    let mut t = input_t.trim().chars();

    let mut result = 0;

    while let Some(s_char) = s.next() {
        let s_char = s_char as i32;
        let t_char = t.next().unwrap() as i32;

        if s_char == t_char {
            continue;
        }

        let (a, b) = if s_char > t_char { (s_char, t_char) } else { (t_char, s_char) };


        let min_d = cmp::min((a - b).abs(), (a - b - 26).abs());

        result+= min_d;
    }

    println!("{}", result);

}
