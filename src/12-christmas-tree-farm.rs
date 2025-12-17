use std::{fs::File, io::{self, BufRead, BufReader}, str::FromStr};

struct Piece {
    cells: Vec<Vec<bool>>
}

impl FromStr for Piece {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells: Vec<_> =
            s.split('\n').map(|row| {
                row.chars().fold(Vec::new(), |mut state, c| {
                    match c {
                        '#' => state.push(true),
                        '.' => state.push(false),
                        _ => ()
                    };
                    state
                })
            })
            .collect();
        Ok(Piece { cells: cells })
    }
}

impl Piece {
    fn bounds_area(&self) -> u64 {
        let height = self.cells.len();
        if height > 0 {
            (self.cells[0].len() * height) as u64
        } else {
            0
        }
    }

    fn true_area(&self) -> u64 {
        self.cells.iter()
            .flatten()
            .map(|&b| if b { 1 } else { 0 })
            .sum()
    }
}

struct Region {
    width: u64,
    height: u64,
    requirements: Vec<u64>
}

impl FromStr for Region {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (area_str, requirements_str) = s.split_once(':').unwrap();
        let (width_str, height_str) = area_str.split_once('x').unwrap();
        let width = width_str.parse::<u64>().unwrap();
        let height = height_str.parse::<u64>().unwrap();
        let requirements: Vec<_> = requirements_str.split(' ')
            .fold(Vec::new(), |mut state, count| {
                if let Ok(count) = count.parse::<u64>() {
                    state.push(count);
                }
                state
            });
        Ok(Region { width: width, height: height, requirements: requirements })
    }
}

fn is_solveable(pieces: &[Piece], region: &Region) -> bool {
    // Lower area bound = perfectly packed
    let region_area = region.width * region.height;
    let used_area: u64 = pieces.iter()
        .zip(region.requirements.clone())
        .map(|(piece, cnt)| piece.true_area() * cnt)
        .sum();
    if used_area > region_area { return false };

    // Upper area bound = 3x3 pieces
    let available_3x3s = region.width / 3 * region.height / 3;
    let piece_cnt: u64 = region.requirements.iter().sum();
    if piece_cnt <= available_3x3s { return true };

    println!("Non-trivial solution");
    false
}

fn main() -> Result<(), io::Error> {
    let f = File::open("12-input.txt")?;
    let reader = BufReader::new(f);

    let lines: Vec<_> = reader.lines().collect::<Result<_,_>>()?;
    let (lines_pieces, lines_regions) = lines.split_at(30);

    let pieces: Vec<_> = lines_pieces.chunks(5)
        .map(|lines| {
            lines[1..lines.len()-1].join("\n").parse::<Piece>().unwrap()
        })
        .collect();

    let regions: Vec<_> = lines_regions.iter()
        .map(|line| {
            line.parse::<Region>().unwrap()
        })
        .collect();

    let result = regions.iter()
        .map(|region| {
            if is_solveable(&pieces, region) {
                1
            } else {
                0
            }
        })
        .sum::<u64>();
    println!("Result: {}", result);
    Ok(())
}