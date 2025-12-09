use std::{fs::File, io::{self, BufRead, BufReader}};

fn parse_range(s: &str) -> (i64, i64) {
    let (low_s, high_s) = s.split_once('-').unwrap();
    (low_s.parse().unwrap(), high_s.parse().unwrap())
}

fn invalid_in_range(low: i64, high: i64) -> i64 {
    let mut total: i64 = 0;
    for i in low..=high {
        let len = i.ilog10() + 1;
        for j in (2..=len).filter(|x| len % x == 0) {
            let split = 10i64.pow(len / j);
            let target = i % split;
            let mut mut_i = i;
            while mut_i > 0 && (mut_i % split) == target {
                mut_i /= split;
            }
            if mut_i == 0 {
                // println!("{} in {}-{}", i, low, high);
                total += i;
                break
            }
        }
    }
    total
}

fn main() -> Result<(), io::Error> {
    let f = File::open("2-input.txt")?;
    let reader = BufReader::new(f);
    let mut ranges = reader.lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(parse_range)
        .collect::<Vec<_>>();
    ranges.sort_unstable();

    let mut total: i64 = 0;
    let mut last: i64 = 0;
    for (low, high) in ranges {
        let low = low.max(last);
        if low <= high { 
            total += invalid_in_range(low, high);
        }
        last = high + 1;
    }
    println!("Result {}", total);
    Ok(())
}