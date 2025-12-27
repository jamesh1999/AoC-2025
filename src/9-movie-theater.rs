use std::cmp::{Ordering, max, min};
use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct ParseVertexError;

impl Display for ParseVertexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot parse vertex")
    }
}

impl Error for ParseVertexError {}

#[derive(Debug)]
struct Vertex {
    x: i64,
    y: i64,
}

impl FromStr for Vertex {
    type Err = ParseVertexError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split(',').map(str::parse::<i64>);

        let Some(Ok(x)) = values.next() else {
            return Err(ParseVertexError);
        };
        let Some(Ok(y)) = values.next() else {
            return Err(ParseVertexError);
        };
        Ok(Vertex { x, y })
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Segment {
    lo: i64,
    hi: i64,
    truncate_lo: bool,
    truncate_hi: bool,
    dist: Option<i64>,
}

impl Segment {
    fn new(start: &Vertex, end: &Vertex) -> Segment {
        Segment {
            lo: min(start.x, end.x),
            hi: max(start.x, end.x),
            truncate_lo: false,
            truncate_hi: false,
            dist: if start.x < end.x { Some(start.y) } else { None },
        }
    }
}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.lo.cmp(&other.lo)
    }
}

impl Debug for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.lo, self.hi)
    }
}

fn rect_area(x1: i64, x2: i64, y1: i64, y2: i64) -> i64 {
    (1 + i64::abs(x1 - x2)) * (1 + i64::abs(y1 - y2))
}

fn rect_from_point(state: &[Segment], v: &Vertex) -> i64 {
    let mut max_area = i64::MIN;

    let mut max_y = i64::MIN;
    for segment in state {
        if segment.hi <= v.x {
            continue;
        }

        let Some(seg_y) = segment.dist else { break };
        if seg_y >= max_y {
            max_y = seg_y;
            if !segment.truncate_hi {
                max_area = max(max_area, rect_area(v.x, segment.hi, v.y, seg_y));
            }
            if !segment.truncate_lo {
                max_area = max(max_area, rect_area(v.x, segment.lo, v.y, seg_y));
            }
        }
    }

    let mut max_y = i64::MIN;
    for segment in state.iter().rev() {
        if segment.lo >= v.x {
            continue;
        }

        let Some(seg_y) = segment.dist else { break };
        if seg_y >= max_y {
            max_y = seg_y;
            if !segment.truncate_hi {
                max_area = max(max_area, rect_area(v.x, segment.hi, v.y, seg_y));
            }
            if !segment.truncate_lo {
                max_area = max(max_area, rect_area(v.x, segment.lo, v.y, seg_y));
            }
        }
    }
    max_area
}

fn update_state(state: &mut Vec<Segment>, incoming: Segment) {
    for i in (0..state.len()).rev() {
        let current = state[i].clone();
        if incoming.lo <= current.lo && current.hi <= incoming.hi {
            state.remove(i);
        } else if current.lo < incoming.lo && incoming.hi < current.hi {
            state.push(Segment {
                hi: incoming.lo,
                truncate_hi: true,
                ..current
            });
            state.push(Segment {
                lo: incoming.hi,
                truncate_lo: true,
                ..current
            });
            state.remove(i);
        } else if current.lo < incoming.lo && incoming.lo < current.hi {
            state[i].hi = incoming.lo;
            state[i].truncate_hi = true;
        } else if current.lo < incoming.hi && incoming.hi < current.hi {
            state[i].lo = incoming.hi;
            state[i].truncate_lo = true;
        }
    }
    state.push(incoming);
    state.sort();
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("9-input.txt")?;
    let reader = BufReader::new(f);
    let vertices: Vec<_> = reader
        .lines()
        .map(|l| Ok(l?.parse::<Vertex>()?))
        .collect::<Result<_, Box<dyn Error>>>()?;

    let mut horizontal_edges: Vec<(usize, usize)> = Vec::new();
    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        if vertices[i].y == vertices[j].y {
            horizontal_edges.push((i, j));
        }
    }
    horizontal_edges.sort_unstable_by_key(|e| vertices[e.0].y);

    let mut max_area: i64 = 0;
    let mut state = vec![Segment {
        lo: i64::MIN,
        hi: i64::MAX,
        truncate_lo: false,
        truncate_hi: false,
        dist: None,
    }];
    for e in horizontal_edges {
        let start = &vertices[e.0];
        let end = &vertices[e.1];
        max_area = max(max_area, rect_from_point(&state, start));
        max_area = max(max_area, rect_from_point(&state, end));
        update_state(&mut state, Segment::new(start, end));
    }

    println!("Result {}", max_area);
    Ok(())
}
