use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> Result<(), io::Error> {
    let f = File::open("7-input.txt")?;
    let reader = BufReader::new(f);

    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;
    let initial_state: Vec<_> = lines[0]
        .chars()
        .map(|c| if c == 'S' { 1 } else { 0 })
        .collect();

    let (final_state, splits) =
        lines[1..]
            .iter()
            .fold((initial_state, 0u64), |(state, mut splits), row| {
                let mut next_state = vec![0; state.len()];
                for (i, c) in row.chars().enumerate() {
                    match (c, state[i]) {
                        (_, 0) => (),
                        ('.', x) => {
                            next_state[i] += x;
                        }
                        ('^', x) => {
                            splits += 1;
                            if i > 0 {
                                next_state[i - 1] += x;
                            }
                            if i < state.len() - 1 {
                                next_state[i + 1] += x;
                            }
                        }
                        _ => (),
                    }
                }
                (next_state, splits)
            });
    let paths: u64 = final_state.iter().sum();
    println!("Result: {} splits, {} paths", splits, paths);
    Ok(())
}
