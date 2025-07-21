use std::io;

fn main() {
    let stdin = io::stdin();

    let mut input = String::with_capacity(12);

    stdin.read_line(&mut input).unwrap();

    let n: u32 = input.trim().parse().unwrap();

    input.clear();
    stdin.read_line(&mut input).unwrap();

    let mut l_values = input.split_whitespace();
    let la = l_values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();
    let lb = l_values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();

    input.clear();
    stdin.read_line(&mut input).unwrap();

    let mut s_values = input.split_whitespace();
    let sa = s_values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();
    let sb = s_values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();

    if n < la || n > lb || n < sa || n > sb {
        println!("impossivel");
    } else {
        println!("possivel");
    }
}
