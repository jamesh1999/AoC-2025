use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> Result<(), io::Error> {
    let f = File::open("7-input.txt")?;
    let reader = BufReader::new(f);

    let mut lines = reader.lines();
    let first = lines.next().transpose()?.unwrap();

    let mut current_state: Vec<_> = first
        .chars()
        .map(|c| if c == 'S' { 1 } else { 0 })
        .collect();
    let mut next_state = vec![0; current_state.len()];
    let mut splits: u64 = 0;

    for line in lines {
        for (i, c) in line?.chars().enumerate() {
            match (c, current_state[i]) {
                (_, 0) => (),
                ('.', x) => {
                    next_state[i] += x;
                }
                ('^', x) => {
                    splits += 1;
                    if i > 0 {
                        next_state[i - 1] += x;
                    }
                    if i < current_state.len() - 1 {
                        next_state[i + 1] += x;
                    }
                }
                _ => (),
            }
        }
        std::mem::swap(&mut current_state, &mut next_state);
        next_state.fill(0);
    }

    let paths: u64 = current_state.iter().sum();
    println!("Result: {} splits, {} paths", splits, paths);
    Ok(())
}
