use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug)]
struct Node {
    x: u32,
    y: u32,
    z: u32,
}

impl Node {
    fn square_dist(a: &Node, b: &Node) -> i64 {
        let dx = i64::from(a.x) - i64::from(b.x);
        let dy = i64::from(a.y) - i64::from(b.y);
        let dz = i64::from(a.z) - i64::from(b.z);
        dx * dx + dy * dy + dz * dz
    }
}

impl FromStr for Node {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',').map(str::parse::<u32>);
        Ok(Node {
            x: it.next().unwrap()?,
            y: it.next().unwrap()?,
            z: it.next().unwrap()?,
        })
    }
}

#[derive(Debug)]
struct DisjointSet<T> {
    nodes: Vec<T>,
    parents: Vec<usize>,
}

impl<T> DisjointSet<T> {
    fn new(nodes: Vec<T>) -> Self {
        let parents = (0..nodes.len()).collect();
        DisjointSet { nodes, parents }
    }

    fn representative_idx(&mut self, i: usize) -> usize {
        let rep = self.parents[i];
        if rep == i {
            rep
        } else {
            let rep = self.representative_idx(rep);
            self.parents[i] = rep;
            rep
        }
    }

    fn union(&mut self, i: usize, j: usize) -> Option<usize> {
        let rep_i = self.representative_idx(i);
        let rep_j = self.representative_idx(j);
        if rep_i != rep_j {
            self.parents[rep_j] = rep_i;
            Some(rep_i)
        } else {
            None
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("8-input.txt")?;
    let reader = BufReader::new(f);

    let nodes: Vec<_> = reader
        .lines()
        .map(|l| Ok(l?.parse::<Node>()?))
        .collect::<Result<_, Box<dyn Error>>>()?;

    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            let sqr_dist = Node::square_dist(&nodes[i], &nodes[j]);
            edges.push((sqr_dist, i, j));
        }
    }
    edges.sort_unstable_by_key(|(d, _, _)| *d);

    let mut ds: DisjointSet<_> = DisjointSet::new(nodes);
    let connections: Vec<_> = edges
        .iter()
        .filter_map(|(_, i, j)| ds.union(*i, *j).map(|_| (*i, *j)))
        .collect();

    let nodes = ds.nodes;
    let last_connection = connections.last().unwrap();
    let last_from = &nodes[last_connection.0];
    let last_to = &nodes[last_connection.1];
    println!("Result {}", u64::from(last_from.x) * u64::from(last_to.x));
    Ok(())
}
