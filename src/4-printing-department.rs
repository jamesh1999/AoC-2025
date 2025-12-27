use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

const NEIGHBOURS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn parse_grid_line(line: &str) -> Vec<bool> {
    line.chars().map(|c| c == '@').collect()
}

fn get_grid_cell(grid: &[Vec<bool>], x: i32, y: i32) -> bool {
    let (Ok(ux), Ok(uy)) = (usize::try_from(x), usize::try_from(y)) else {
        return false;
    };
    grid.get(uy)
        .and_then(|row| row.get(ux))
        .copied()
        .unwrap_or(false)
}

fn count_surrounding(grid: &[Vec<bool>], x: i32, y: i32) -> i32 {
    NEIGHBOURS
        .iter()
        .map(|(dx, dy)| get_grid_cell(grid, x + dx, y + dy))
        .filter(|&p| p)
        .count() as i32
}

fn update_grid(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let height = grid.len();
    let width = grid[0].len();
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    let x = x as i32;
                    let y = y as i32;
                    get_grid_cell(grid, x, y) && (count_surrounding(grid, x, y) >= 4)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn count_all_true(grid: &[Vec<bool>]) -> usize {
    grid.iter().flatten().filter(|&&b| b).count()
}

fn main() -> Result<(), io::Error> {
    let f = File::open("4-input.txt")?;
    let reader = BufReader::new(f);
    let original_grid: Vec<_> = reader
        .lines()
        .map(|line| Ok(parse_grid_line(&line?)))
        .collect::<Result<_, io::Error>>()?;

    let mut current_grid = original_grid.clone();
    let last_grid = loop {
        let next_grid = update_grid(&current_grid);
        if next_grid == current_grid {
            break next_grid;
        }
        current_grid = next_grid;
    };

    println!(
        "Result: {}",
        count_all_true(&original_grid) - count_all_true(&last_grid)
    );
    Ok(())
}
