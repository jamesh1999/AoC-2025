use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn apply(&self, values: &[u64]) -> u64 {
        match self {
            Operation::Add => values.iter().sum(),
            Operation::Mul => values.iter().product(),
        }
    }
}

impl From<&char> for Operation {
    fn from(value: &char) -> Self {
        match value {
            '+' => Operation::Add,
            '*' => Operation::Mul,
            _ => panic!("Unknown operation"),
        }
    }
}

struct Problem {
    operation: Operation,
    values: Vec<u64>,
}

impl Problem {
    fn solve(&self) -> u64 {
        // println!("{} {:?}", op, xs);
        self.operation.apply(&self.values)
    }
}

fn transpose_chars(input: Vec<String>) -> Vec<Vec<char>> {
    let height = input[0].len();
    let mut input_iters: Vec<_> = input.iter().map(|s| s.chars()).collect();

    (0..height)
        .map(|_| {
            input_iters
                .iter_mut()
                .map(|it| it.next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn split_problems(input: &[Vec<char>]) -> Vec<Problem> {
    let mut problems = Vec::new();
    input
        .iter()
        .rev()
        .fold(Vec::new(), |mut acc: Vec<u64>, cs| {
            let Ok(next) = cs[..cs.len() - 1]
                .iter()
                .collect::<String>()
                .trim()
                .parse::<u64>()
            else {
                return acc;
            };
            match cs.last() {
                None => panic!("Oh no!"),
                Some(' ') => {
                    acc.push(next);
                    acc
                }
                Some(op) => {
                    acc.push(next);
                    problems.push(Problem {
                        operation: op.into(),
                        values: acc,
                    });
                    Vec::new()
                }
            }
        });
    problems
}

fn main() -> Result<(), io::Error> {
    let f = File::open("6-input.txt")?;
    let reader = BufReader::new(f);

    let inputs = reader.lines().collect::<Result<Vec<_>, _>>()?;

    let inputs_t = transpose_chars(inputs);
    let total: u64 = split_problems(&inputs_t).iter().map(Problem::solve).sum();
    println!("Result {}", total);
    Ok(())
}
