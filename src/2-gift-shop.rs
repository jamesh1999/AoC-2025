use std::{
    cmp::max,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn parse_range(s: &str) -> (i64, i64) {
    let (low_s, high_s) = s.split_once('-').unwrap();
    (low_s.parse().unwrap(), high_s.parse().unwrap())
}

fn is_invalid(value: i64) -> bool {
    let len = value.ilog10() + 1;
    for j in (1..len).filter(|x| len % x == 0) {
        let split = 10_i64.pow(j);
        let target = value % split;
        let mut mut_value = value;
        while mut_value > 0 && (mut_value % split) == target {
            mut_value /= split;
        }
        if mut_value == 0 {
            return true;
        }
    }
    false
}

fn main() -> Result<(), io::Error> {
    let f = File::open("2-input.txt")?;
    let reader = BufReader::new(f);
    let range_line = reader.lines().next().unwrap()?;
    let mut ranges: Vec<_> = range_line.split(",").map(parse_range).collect();
    ranges.sort_unstable();

    let mut total: i64 = 0;
    let mut last: i64 = 0;
    for (low, high) in ranges {
        let low = max(low, last);
        last = max(high + 1, last);
        total += (low..last).filter(|x| is_invalid(*x)).sum::<i64>();
    }
    println!("Result {}", total);
    Ok(())
}
