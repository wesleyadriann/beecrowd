use std::io;

fn main() {
    let mut input = String::with_capacity(8);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut values = input.trim().split("/");

    let dia = values.next().unwrap();
    let mes = values.next().unwrap();

    let mut data = String::from(mes);
    data.push_str(&dia);

    let data_n = data.parse::<u32>().unwrap();

    let result = match data_n {
        101..=119 => "capricornio",
        120..=218 => "aquario",
        219..=320 => "peixes",
        321..=420 => "aries",
        421..=520 => "touro",
        521..=620 => "gemeos",
        621..=722 => "cancer",
        723..=822 => "leao",
        823..=922 => "virgem",
        923..=1022 => "libra",
        1023..=1121 => "escorpiao",
        1122..=1221 => "sagitario",
        _ => "capricornio",
    };

    println!("{}", result);
}
