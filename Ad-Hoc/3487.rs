use std::io;

// TODO: wait for update of the problem description
fn main() {
    let stdin = io::stdin();

    let mut string = String::with_capacity(6);
    let mut sub_str = String::with_capacity(6);

    stdin.read_line(&mut string).unwrap();
    stdin.read_line(&mut sub_str).unwrap();

    let sub_str = sub_str.trim();

    while string.contains(&sub_str) {
        string = string.replacen(&sub_str, "", 1);
    }

    println!("{}", string.trim());
}
