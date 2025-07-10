use std::io;

fn main() {
    let stdin = io::stdin();

    let mut input = String::with_capacity(32);
    let mut input_target = String::with_capacity(32);

    stdin.read_line(&mut input).unwrap();
    stdin.read_line(&mut input_target).unwrap();

    let values = input.split_whitespace().enumerate();
    let cloned_values = values.clone();
    let target = input_target.trim().parse::<f64>().unwrap();

    let mut result = Vec::new();

    for (i, value) in values {
        let n = value.parse::<f64>().unwrap();

        let next_values = cloned_values.clone().skip(i + 1);
        for (j, next) in next_values {
            let m = next.parse::<f64>().unwrap();
            if (n + m) == target {
                result.push((i, j));
            }
        }
    }

    if result.is_empty() {
        println!("null value");
        return;
    }
    let target_result = if target.fract() > 0.0 {
        format!("{:.1}", target)
    } else {
        format!("{:.0}", target)
    };
    for (i, j) in result.into_iter() {
        println!("{} {} {}", i, j, target_result);
    }
}
