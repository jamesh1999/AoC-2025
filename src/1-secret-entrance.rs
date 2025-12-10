use std::fs::File;
use std::io::{self, BufRead, BufReader};

const DIAL_SIZE: i64 = 100;

fn parse_turn(s: &str) -> i64 {
    let (dir, num) = s.split_at(1);
    let sign = match dir {
        "L" => -1,
        "R" => 1,
        _ => panic!("Bad input direction"),
    };
    sign * num.parse::<i64>().unwrap()
}

fn main() -> Result<(), io::Error> {
    let f = File::open("1-input.txt")?;
    let br = BufReader::new(f);
    let mut position = 50;
    let mut zeroes = 0;
    for line in br.lines() {
        let line = line?;
        let resulting_position = position + parse_turn(&line);

        zeroes += (resulting_position / DIAL_SIZE).abs();
        if position > 0 && resulting_position <= 0 {
            zeroes += 1;
        }
        position = resulting_position.rem_euclid(DIAL_SIZE);
        // println!("Move: {}, (P: {}, C: {})", line, position, zeroes);
    }
    println!("Result: {}", zeroes);
    Ok(())
}
