use std::io;

fn main() {
    let  stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    input.clear();

    stdin.read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let mut result2 = 0;
    let mut result3 = 0;
    let mut result4 = 0;
    let mut result5 = 0;

    while let Some(value) = values.next() {
        let value = value.parse::<u32>().unwrap();

        if value % 2 == 0 {
            result2 += 1;
        }
        if value % 3 == 0 {
            result3 += 1;
        }
        if value % 4 == 0 {
            result4 += 1;
        }
        if value % 5 == 0 {
            result5 += 1;
        }
    }

    println!("{} Multiplo(s) de 2", result2);
    println!("{} Multiplo(s) de 3", result3);
    println!("{} Multiplo(s) de 4", result4);
    println!("{} Multiplo(s) de 5", result5);
}
