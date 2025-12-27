use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn max_joltage(s: &str) -> u64 {
    let values: Vec<_> = s
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let max_idx = |xs: &[u32]| {
        
        xs.iter()
            .enumerate()
            .fold(0, |max, (i, &x)| if x > xs[max] { i } else { max })
    };

    let (_, total) = (0..12).rev().fold((0, 0u64), |(start_idx, total), i| {
        let next_idx = start_idx + max_idx(&values[start_idx..values.len() - i]);
        (next_idx + 1, 10 * total + values[next_idx] as u64)
    });
    total
}

fn main() -> Result<(), io::Error> {
    let f = File::open("3-input.txt")?;
    let reader = BufReader::new(f);

    let total: u64 = reader.lines()
        .map(|l| {
            max_joltage(&l.unwrap())
        })
        .sum();
    println!("Result: {}", total);
    Ok(())
}
