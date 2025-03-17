use std::io;

fn main() {
    let mut first_value = String::new();
    let mut second_value = String::new();

    io::stdin()
        .read_line(&mut first_value)
        .expect("Failed to read line");

    let a = first_value.trim().parse::<i32>().expect("insert a number!");
    
    io::stdin()
        .read_line(&mut second_value)
        .expect("Failed to read line");

    let b =  second_value.trim().parse::<i32>().expect("insert a number!");

    println!("X = {}", a + b);
}
