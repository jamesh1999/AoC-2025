use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Clone)]
enum GraphNode {
    Output,
    Branch(Vec<String>),
}

fn count_paths(
    node: &str,
    nodes: &HashMap<String, GraphNode>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if let Some(&count) = memo.get(node) {
        return count;
    }

    let count = match nodes.get(node) {
        None => 0,
        Some(GraphNode::Output) => 1,
        Some(GraphNode::Branch(children)) => {
            children.iter().map(|c| count_paths(c, nodes, memo)).sum()
        }
    };
    memo.insert(node.to_string(), count);
    count
}

fn main() -> Result<(), io::Error> {
    let f = File::open("11-input.txt")?;
    let reader = BufReader::new(f);

    let nodes: HashMap<String, GraphNode> = reader
        .lines()
        .map(|line| {
            let line = line?;
            let parts: Vec<_> = line.split(' ').collect();
            let name = parts[0][..3].to_string();
            if parts[1] == "out" {
                Ok((name, GraphNode::Output))
            } else {
                let branches: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                Ok((name, GraphNode::Branch(branches)))
            }
        })
        .collect::<Result<_, io::Error>>()?;

    let svr_paths = count_paths("svr", &nodes, &mut HashMap::new());
    let missing_fft_paths = count_paths(
        "svr",
        &(nodes
            .clone()
            .into_iter()
            .filter(|(k, _)| k != "fft")
            .collect()),
        &mut HashMap::new(),
    );
    let missing_dac_paths = count_paths(
        "svr",
        &(nodes
            .clone()
            .into_iter()
            .filter(|(k, _)| k != "dac")
            .collect()),
        &mut HashMap::new(),
    );
    let missing_both_paths = count_paths(
        "svr",
        &(nodes
            .clone()
            .into_iter()
            .filter(|(k, _)| k != "fft" && k != "dac")
            .collect()),
        &mut HashMap::new(),
    );

    println!(
        "Result {}",
        svr_paths + missing_both_paths - missing_fft_paths - missing_dac_paths
    );

    Ok(())
}
