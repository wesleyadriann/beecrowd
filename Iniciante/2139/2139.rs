use std::io::{self, BufRead, BufReader};

fn main() {
    const DAYS_IN_MONTH: [u32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 25];

    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut values = line.split_whitespace();

        let mouth = values.next()
            .and_then(|x| x.parse::<u32>().ok())
            .unwrap();
        let day = values.next()
            .and_then(|x| x.parse::<u32>().ok())
            .unwrap();

        if mouth == 12 {
            if day == 24 {
                println!("E vespera de natal!");
                continue;
            }

            if day == 25 {
                println!("E natal!");
                continue;
            }

            if day > 25 {
                println!("Ja passou!");
                continue;
            }
        }

        let mut result: u32 = 0;

        for i in (mouth as usize - 1)..12 {
            result += DAYS_IN_MONTH[i];
        }
        result -= day;

        println!("Faltam {} dias para o natal!", result);
    }
}
