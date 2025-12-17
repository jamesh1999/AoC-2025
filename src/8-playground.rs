use std::{collections::HashMap, fs::File, hash::Hash,io::{self, BufRead, BufReader}};

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
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
        dx*dx + dy*dy + dz*dz
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let coords: Vec<_> = value.split(',').map(|s| s.parse::<u32>().unwrap()).collect();
        Node {
            x: coords[0],
            y: coords[1],
            z: coords[2]
        }
    }
}

#[derive(Debug, Clone)]
struct Edge {
    a: Node,
    b: Node
}

#[derive(Debug)]
struct DisjointSet<T> {
    parents: HashMap<T, Option<T>>
}

impl<T: Eq + Hash> FromIterator<T> for DisjointSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        DisjointSet {
            parents: iter.into_iter().map(|x| (x, None)).collect()
        }
    }
}

impl<T: Eq + Hash + Clone> DisjointSet<T> {
    fn representative(&mut self, x: &T) -> Option<T> {
        match self.parents.get(x)?.clone() {
            None => Some(x.clone()),
            Some(p) => {
                let rep = self.representative(&p)?;
                self.parents.insert(x.clone(), Some(rep.clone()));
                Some(rep)
            }
        }
    }

    fn union(&mut self, a: &T, b: &T) -> Option<T> {
        let rep_a = self.representative(a).unwrap();
        let rep_b = self.representative(b).unwrap();
        if rep_a != rep_b {
            self.parents.insert(rep_a, Some(rep_b.clone()));
            Some(rep_b)
        } else {
            None
        }
    }
}

fn main() -> Result<(), io::Error> {
    let f = File::open("8-input.txt")?;
    let reader = BufReader::new(f);

    let nodes: Vec<Node> = reader.lines()
        .map(|lr| lr.map(|l| l.as_str().into()))
        .collect::<Result<_, _>>()?;
    let node_cnt = nodes.len();

    let mut all_edges: Vec<_> =
        (0..node_cnt).flat_map(|a| {
            let na = &nodes[a];
            (a+1..node_cnt).map(|b| {
                let nb = &nodes[b];
                let sqr_dist = Node::square_dist(na, nb);
                (sqr_dist, Edge {a: na.clone(), b: nb.clone()})
            })
        })
        .filter(|(_, e)| e.a != e.b)
        .collect();
    all_edges.sort_unstable_by_key(|(d, _)| *d);

    let mut ds: DisjointSet<_> = nodes.clone().into_iter().collect();
    let connections: Vec<_> = all_edges.iter()
        .filter_map(|(_, e)| ds.union(&e.a, &e.b).map(|_|e.clone()))
        .collect();

    let last_connection = connections.last().unwrap();
    println!("Result {}", u64::from(last_connection.a.x) * u64::from(last_connection.b.x));
    Ok(())
}