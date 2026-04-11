use std::io;

fn main() {
    const DAYS_IN_MONTH: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let stdin = io::stdin();

    let mut input_1 = String::new();
    let mut input_2 = String::new();

    stdin.read_line(&mut input_1).unwrap();
    stdin.read_line(&mut input_2).unwrap();

    let mut dates_1 = input_1.split_whitespace();
    let mut dates_2 = input_2.split_whitespace();

    let d1: u32 = dates_1.next().and_then(|x| x.parse().ok()).unwrap();
    let m1: u32 = dates_1.next().and_then(|x| x.parse().ok()).unwrap();

    let d2: u32 = dates_2.next().and_then(|x| x.parse().ok()).unwrap();
    let m2: u32 = dates_2.next().and_then(|x| x.parse().ok()).unwrap();

    if m1 == m2 {
        println!("{}", d2 - d1);
        return;
    }


    let mut result: u32 = 0;

    for i in (m1 as usize - 1)..(m2 as usize - 1) {
        result += DAYS_IN_MONTH[i];
    }

    result -= d1;
    result += d2;

    println!("{}", result);

}
