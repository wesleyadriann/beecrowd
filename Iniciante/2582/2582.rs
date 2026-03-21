use std::io::{self, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(io::stdin());

    for line in reader.lines().skip(1) {
        let line = line.unwrap();
        let mut values = line.split_whitespace();

        let x = values.next()
            .and_then(|x| x.parse::<u32>().ok())
            .unwrap();
        let y = values.next()
            .and_then(|x| x.parse::<u32>().ok())
            .unwrap();


        let c = x + y;

        let music = match c {
            0 => "PROXYCITY",
            1 => "P.Y.N.G.",
            2 => "DNSUEY!",
            3 => "SERVERS",
            4 => "HOST!",
            5 => "CRIPTONIZE",
            6 => "OFFLINE DAY",
            7 => "SALT",
            8 => "ANSWER!",
            9 => "RAR?",
            10 => "WIFI ANTENNAS",
            _ => ""
        };

        println!("{}", music);
    }
}
