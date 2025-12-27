use std::convert::Infallible;
use std::error::Error;
use std::num::ParseIntError;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

struct Piece {
    cells: Vec<Vec<bool>>,
}

impl FromStr for Piece {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .split('\n')
            .map(|row| {
                row.chars()
                    .filter_map(|c| match c {
                        '#' => Some(true),
                        '.' => Some(false),
                        _ => None,
                    })
                    .collect()
            })
            .collect();
        Ok(Piece { cells })
    }
}

impl Piece {
    #[allow(dead_code)]
    fn bounding_area(&self) -> u64 {
        let height = self.cells.len();
        if height > 0 {
            (self.cells[0].len() * height) as u64
        } else {
            0
        }
    }

    fn true_area(&self) -> u64 {
        self.cells.iter().flatten().filter(|&&b| b).count() as u64
    }
}

struct Region {
    width: u64,
    height: u64,
    requirements: Vec<u64>,
}

impl FromStr for Region {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (area_str, requirements_str) = s.split_once(':').unwrap();
        let (width_str, height_str) = area_str.split_once('x').unwrap();
        let width = width_str.parse::<u64>()?;
        let height = height_str.parse::<u64>()?;
        let requirements = requirements_str
            .split(' ')
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        Ok(Region {
            width,
            height,
            requirements,
        })
    }
}

fn is_solveable(pieces: &[Piece], region: &Region) -> bool {
    // Lower area bound = perfectly packed
    let region_area = region.width * region.height;
    let used_area: u64 = pieces
        .iter()
        .zip(region.requirements.iter())
        .map(|(piece, cnt)| piece.true_area() * cnt)
        .sum();
    if used_area > region_area {
        return false;
    };

    // Upper area bound = 3x3 pieces
    let available_3x3s = region.width / 3 * region.height / 3;
    let piece_cnt: u64 = region.requirements.iter().sum();
    if piece_cnt <= available_3x3s {
        return true;
    };

    println!("Non-trivial solution");
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("12-input.txt")?;
    let reader = BufReader::new(f);

    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;
    let (lines_pieces, lines_regions) = lines.split_at(30);

    let pieces: Vec<_> = lines_pieces
        .chunks(5)
        .map(|lines| Ok(lines[1..lines.len() - 1].join("\n").parse::<Piece>()?))
        .collect::<Result<_, Box<dyn Error>>>()?;

    let regions: Vec<_> = lines_regions
        .iter()
        .map(|line| Ok(line.parse::<Region>()?))
        .collect::<Result<_, Box<dyn Error>>>()?;

    let result: u64 = regions
        .iter()
        .filter(|region| is_solveable(&pieces, region))
        .count() as u64;
    println!("Result: {}", result);
    Ok(())
}
