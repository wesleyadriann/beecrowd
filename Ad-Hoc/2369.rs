use std::io;

fn main() {
    let mut input = String::with_capacity(8);

    io::stdin().read_line(&mut input).unwrap();

    let n: u32 = input.trim()
        .parse()
        .unwrap();


    if n > 100 {
        let result = ((n - 100) * 5) + 167;
        println!("{}", result);
        return
    }

    if n > 30 {
        let result = ((n - 30) * 2) + 27;
        println!("{}", result);
        return
    }

     if n > 10 {
         let result = n - 10 + 7;
         println!("{}", result);
         return
     }

    println!("7");
}
