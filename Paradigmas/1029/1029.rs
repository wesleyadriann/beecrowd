use std::io::{self, BufRead, BufReader};

fn main() {
    let mut fib_result = [-1; 39];
    let mut fib_stack = [0; 39];

    fib_result[0] = 0;
    fib_result[1] = 1;

    fib_stack[0] = 1;
    fib_stack[1] = 1; 

    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines().skip(1) {
        let line = line.unwrap();
        let value: usize = line.parse().unwrap();

        let (fib, call) = fib(value, &mut fib_result, &mut fib_stack);

        println!("fib({}) = {} calls = {}", value, call - 1, fib);
    }
}

fn fib(n: usize, fib_result: &mut [i32], fib_stack: &mut [u32]) -> (i32, u32) {
    if fib_result[n] == -1 {
        let (fib_left, call_left) = fib(n - 1, fib_result, fib_stack);
        let (fib_right, call_right) = fib(n - 2, fib_result, fib_stack);
 
        fib_result[n] = fib_left + fib_right;
        fib_stack[n] = call_left + call_right + 1;
    }

    (fib_result[n], fib_stack[n])
}


// fn main() {
//     let stdin = io::stdin();
//     let reader = BufReader::new(stdin.lock());
//
//     for line in reader.lines().skip(1) {
//         let line = line.unwrap();
//         let value: u32 = line.parse().unwrap();
//
//         let (fib, call) = fib(value, 0);
//
//         println!("fib({}) = {} calls = {}", value, call - 1, fib);
//     }
// }
//
// fn fib(n: u32, call: u32) -> (u32, u32) {
//     if n <= 1 {
//         return (n, call + 1)
//     }
//
//     let (fib_left, call_left) = fib(n - 1, call);
//     let (fib_right, call_right) = fib(n - 2, call);
//
//     return (fib_left + fib_right, call_right + call_left + 1);
// }
