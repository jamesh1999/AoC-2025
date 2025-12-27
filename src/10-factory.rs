use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Machine {
    #[allow(dead_code)]
    lights: Vec<u64>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u64>,
}

impl FromStr for Machine {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let groups: Vec<_> = s.split(' ').collect();

        let lights_part = groups[0];
        let lights = lights_part[1..lights_part.len() - 1]
            .chars()
            .map(|c| if c == '#' { 1 } else { 0 })
            .collect();

        let buttons: Vec<_> = (1..groups.len() - 1)
            .map(|i| parse_button_group(groups[i]))
            .collect::<Result<_, _>>()?;

        let joltages_part = groups[groups.len() - 1];
        let joltages: Vec<_> = joltages_part[1..joltages_part.len() - 1]
            .split(',')
            .map(|j_s| j_s.parse::<u64>())
            .collect::<Result<_, _>>()?;

        Ok(Machine {
            lights,
            buttons,
            joltages,
        })
    }
}

fn parse_button_group(s: &str) -> Result<Vec<usize>, ParseIntError> {
    s[1..s.len() - 1]
        .split(',')
        .map(str::parse::<usize>)
        .collect()
}

fn parity_solve(machine: &Machine, target: &[u64]) -> Vec<u64> {
    let mut solutions: Vec<u64> = Vec::new();
    for selection in 0..(1 << machine.buttons.len()) {
        let mut parity = target
            .iter()
            .rev()
            .fold(0, |state, x| (state << 1) | (x & 1));
        for (i, button) in machine.buttons.iter().enumerate() {
            if selection & (1 << i) != 0 {
                parity = button.iter().fold(parity, |parity, j| parity ^ (1 << j));
            }
        }
        if parity == 0 {
            solutions.push(selection);
        }
    }
    solutions
}

fn minimum_solve(machine: &Machine) -> u64 {
    fn apply_button<'a>(
        machine: &Machine,
        button_idx: usize,
        current: &'a mut Vec<u64>,
    ) -> Option<&'a mut Vec<u64>> {
        for i in &machine.buttons[button_idx] {
            if current[*i] == 0 {
                return None;
            }
            current[*i] -= 1;
        }
        Some(current)
    }

    fn recurse(machine: &Machine, target: &[u64], max_cnt: i64) -> Option<u64> {
        if target.iter().sum::<u64>() == 0 {
            return Some(0);
        };
        if max_cnt <= 0 {
            return None;
        };

        let parity_solutions = parity_solve(machine, target);

        let mut min_presses = None;
        let mut current_storage = target.to_vec();
        for parity_sln in parity_solutions {
            current_storage.copy_from_slice(target);
            let current =
                (0..machine.buttons.len()).fold(Some(&mut current_storage), |state, i| {
                    if parity_sln & (1 << i) != 0 {
                        apply_button(machine, i, state?)
                    } else {
                        state
                    }
                });
            let Some(current) = current else { continue };
            let parity_ones = parity_sln.count_ones() as i64;

            let new_max_cnt = (match min_presses {
                None => max_cnt,
                Some(ms) => min(ms as i64, max_cnt),
            } - parity_ones)
                / 2;
            let new_target: Vec<_> = current.iter().map(|&x| x / 2).collect();
            let Some(sln) = recurse(machine, &new_target, new_max_cnt) else {
                continue;
            };
            let presses = parity_ones as u64 + 2u64 * sln;
            min_presses = Some(match min_presses {
                None => presses,
                Some(ms) => min(ms, presses),
            });
        }
        min_presses
    }

    recurse(machine, &machine.joltages, i64::MAX).unwrap_or(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("10-input.txt")?;
    let reader = BufReader::new(f);

    let machines: Vec<_> = reader
        .lines()
        .map(|l| Ok(l?.parse::<Machine>()?))
        .collect::<Result<_, Box<dyn Error>>>()?;
    println!("Parsed {} machines", machines.len());

    let presses: u64 = machines.iter().map(minimum_solve).sum();
    println!("Result: {}", presses);
    Ok(())
}
