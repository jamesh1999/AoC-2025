use std::cmp::min;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;
use std::u64;

#[derive(Debug)]
struct Machine {
    lights: Vec<u64>,
    buttons: Vec<Vec<u64>>,
    joltages: Vec<u64>
}

impl FromStr for Machine {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let groups: Vec<_> = s.split(' ').collect();
        let lights_part = groups[0];
        let lights = lights_part[1..lights_part.len()-1]
            .chars()
            .rev()
            .map(|c| if c == '#' { 1 } else { 0 })
            .collect();

        let buttons: Vec<_> = (1..groups.len()-1)
            .map(|i| parse_button_group(groups[i]))
            .collect();

        let joltages_part = groups[groups.len()-1];
        let joltages: Vec<_> = joltages_part[1..joltages_part.len()-1]
            .split(',')
            .map(|j_s| j_s.parse::<u64>().unwrap())
            .collect();

        Ok(Machine { lights: lights, buttons: buttons , joltages: joltages })
    }
}

fn parse_button_group(s: &str) -> Vec<u64> {
    s[1..s.len()-1].split(',')
        .map(|x_s| x_s.parse::<u64>().unwrap())
        .collect()
}

fn parity_solve(machine: &Machine, target: &Vec<u64>) -> Vec<u64> {
    let mut solutions: Vec<u64> = Vec::new();
    for selection in 0..(1 << machine.buttons.len()) {
        let mut parity = target.iter().rev()
            .fold(0, |state, x| (state << 1) + (x & 1));
        for (i, button) in machine.buttons.iter().enumerate() {
            if selection & (1 << i) != 0 {
                for j in button {
                    parity ^= 1 << j;
                }
            }
        }
        if parity == 0 {
            solutions.push(selection);
        }
    }
    solutions
}

fn minimum_solve(machine: &Machine) -> u64 {
    fn apply_button(machine: &Machine, button_idx: usize, mut current: Vec<u64>) -> Option<Vec<u64>> {
        for i in &machine.buttons[button_idx] {
            if current[*i as usize] == 0 {
                return None;
            }
            current[*i as usize] -= 1;
        }
        Some(current)
    }

    fn recurse(machine: &Machine, target: &Vec<u64>, max_cnt: i64) -> Option<u64> {
        if target.iter().sum::<u64>() == 0 { return Some(0) };
        if max_cnt <= 0 { return None };

        let parity_solutions = parity_solve(machine, target);
        let mut min_solution = None;
        for parity_sln in parity_solutions {
            let current = (0..machine.buttons.len())
                .fold(Some(target.clone()), |state, i| {
                    if parity_sln & (1 << i) != 0 {
                        apply_button(machine, i, state?)
                    } else {
                        state
                    }
                });
            let Some(current) = current else { continue };
            let parity_ones = parity_sln.count_ones() as i64;

            let new_max_cnt =
                (match min_solution {
                    None => max_cnt,
                    Some(ms) => min(ms as i64, max_cnt)
                } - parity_ones) / 2;
            let new_target = current.iter().map(|&x| x / 2).collect();
            let sln = recurse(machine, &new_target, new_max_cnt);
            let Some(sln) = sln else { continue };
            min_solution =
                match min_solution {
                    None => Some(parity_ones as u64 + 2u64 * sln),
                    Some(ms) => Some(min(ms, parity_ones as u64 + 2u64 * sln))
                };
        }
        min_solution
    }

    let result = recurse(machine, &machine.joltages, i64::MAX).unwrap_or(0);
    println!("Result: {}", result);
    result
}

fn main() -> Result<(), io::Error> {
    let f = File::open("10-input.txt")?;
    let reader = BufReader::new(f);

    let machines: Vec<_> = reader.lines()
        .map(|l| l?.parse::<Machine>())
        .collect::<Result<_,_>>()?;
    println!("Parsed {} machines: \n {:?}", machines.len(), machines);

    let presses: u64 = machines.iter()
        .map(|m| minimum_solve(&m))
        .sum();

    println!("Result: {}", presses);
    Ok(())
}
