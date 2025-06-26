use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::with_capacity(6);

    let mut biggest = 0;
    let mut biggest_index = 0;
    let mut andares = [0; 3];

    for i in 0..3 {
        input.clear();

        stdin.read_line(&mut input).unwrap();
        let andar = input.trim().parse::<i32>().unwrap();

        if andar > biggest {
            biggest_index = i;
            biggest = andar;
        }
        andares[i as usize] = andar;
    }

    if ((andares[0] + andares[1]) > biggest && biggest_index == 2)
        || ((andares[1] + andares[2]) > biggest && biggest_index == 0)
    {
        biggest_index = 1;
    }

    let mut result: i32 = 0;

    for (i, value) in andares.iter().enumerate() {
        result += (biggest_index - i as i32).abs() * 2 * value;
    }

    println!("{}", result);
}
