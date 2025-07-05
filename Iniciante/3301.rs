use std::io;

fn main() {
    let mut input = String::with_capacity(8);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut values = input.split_whitespace();

    let h = values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();
    let z = values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();
    let l = values.next().and_then(|x| x.parse::<u32>().ok()).unwrap();

    let mid = (h + z + l) / 3;

    if mid == h {
        println!("huguinho");
    } else if mid == z {
        println!("zezinho");
    } else {
        println!("luisinho");
    }
}
