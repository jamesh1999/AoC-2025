use std::{cmp::{max, min, Ordering}, fs::File, i64, io::{self, BufRead, BufReader, ErrorKind}, fmt::Debug, str::FromStr};

#[derive(Clone, Copy, Debug)]
struct Vertex {
    x: i64,
    y: i64
}

impl FromStr for Vertex {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(',')
            .map(|s| s.parse::<i64>())
            .collect();

        let err = io::Error::new(ErrorKind::Other, "Unable to parse vertex");
        let Some(&Ok(x)) = parts.get(0) else { return Err(err) };
        let Some(&Ok(y)) = parts.get(1) else { return Err(err) };
        Ok(Vertex { x: x, y: y})
    }
}

#[derive(Debug)]
struct Edge {
    a: Vertex,
    b: Vertex
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Segment {
    lo: i64,
    hi: i64,
    truncate_lo: bool,
    truncate_hi: bool,
    dist: Option<i64>
}

impl Segment {
    fn new(e: Edge) -> Segment {
        Segment {
            lo: min(e.a.x, e.b.x),
            hi: max(e.a.x, e.b.x),
            truncate_lo: false,
            truncate_hi: false,
            dist:
                if e.a.x < e.b.x { Some(e.a.y) }
                else { None }
        }
    }
}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hi <= other.lo {
            Some(Ordering::Less)
        } else if other.hi <= self.lo {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Debug for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({}, {})", self.lo, self.hi).as_str())
    }
}

fn rect_area(x1: i64, x2: i64, y1: i64, y2: i64) -> i64 {
    (1 + i64::abs(x1 - x2)) * (1 + i64::abs(y1 - y2))
}

fn rect_from_point(state: &Vec<Segment>, v: &Vertex) -> i64 {
    let mut max_area = i64::MIN;

    let mut max_y = i64::MIN;
    for segment in state {
        if segment.hi <= v.x { continue }

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
        if segment.lo >= v.x { continue }

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
        let current = state[i];
        if incoming.lo <= current.lo && current.hi <= incoming.hi {
            state.remove(i);
        } else if current.lo < incoming.lo && incoming.hi < current.hi {
            state.push(Segment { hi: incoming.lo, truncate_hi: true, ..current });
            state.push(Segment { lo: incoming.hi, truncate_lo: true, ..current });
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

fn main() -> Result<(), io::Error> {
    let f = File::open("9-input.txt")?;
    let reader = BufReader::new(f);
    let vertices: Vec<_> = reader.lines()
        .map(|l| l?.parse::<Vertex>())
        .collect::<Result<_, _>>()?;
    
    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        edges.push(Edge {
            a: vertices[i],
            b: vertices[j]
        });
    }
    edges.sort_unstable_by_key(|e| e.a.y + e.b.y);

    let horizontal_edges: Vec<_> = edges.into_iter()
        .filter(|e| e.a.y == e.b.y)
        .collect();

    let mut max_area: i64 = 0;
    let mut state: Vec<Segment> = Vec::new();
    state.push(Segment { lo: i64::MIN, hi: i64::MAX, truncate_lo: false, truncate_hi: false, dist: None });
    for e in horizontal_edges {
        max_area = max(max_area, rect_from_point(&state, &e.a));
        max_area = max(max_area, rect_from_point(&state, &e.b));
        update_state(&mut state, Segment::new(e));
    }

    println!("Result {}", max_area);
    Ok(())
}