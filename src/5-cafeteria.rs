use std::{
    cmp::max,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn parse_range(s: &str) -> (u64, u64) {
    let (s1, s2) = s.split_once('-').unwrap();
    (s1.parse().unwrap(), s2.parse().unwrap())
}

fn main() -> Result<(), io::Error> {
    let f = File::open("5-input.txt")?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines().map_while(Result::ok);

    let mut ranges: Vec<_> = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| parse_range(&l))
        .collect();
    ranges.sort_unstable();

    let mut ids: Vec<u64> = lines.map(|l| l.parse::<u64>().unwrap()).collect();
    ids.sort_unstable();

    let mut total = 0;

    // let mut range_idx = 0;
    // let mut id_idx = 0;
    // while id_idx < ids.len() && range_idx < ranges.len() {
    //     let (low, high) = ranges[range_idx];
    //     if ids[id_idx] < low {
    //         id_idx += 1;
    //     } else if ids[id_idx] <= high {
    //         id_idx += 1;
    //         total += 1;
    //     } else {
    //         range_idx += 1;
    //     }
    // }

    let mut max_seen = 0;
    for (low, high) in ranges {
        let new_low = max(low, max_seen + 1);
        max_seen = max(high, max_seen);
        total += 1 + max_seen - new_low;
    }
    println!("Result {}", total);
    Ok(())
}
