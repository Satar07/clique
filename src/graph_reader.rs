use petgraph::graph::UnGraph;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphErr {
    #[error("invalid DIMACS format: {0}")]
    ParseError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<&str> for GraphErr {
    fn from(s: &str) -> Self {
        GraphErr::ParseError(s.into())
    }
}

pub fn read_dimacs(path: &str) -> Result<UnGraph<(), ()>, GraphErr> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut graph = UnGraph::new_undirected();
    let mut nodes = Vec::new();

    for line in reader.lines() {
        let line = line?;
        // e 10 2
        if line.starts_with('e') {
            let stuff: Vec<&str> = line.split_ascii_whitespace().collect();
            if stuff.len() != 3 {
                return Err(GraphErr::ParseError("bad edge format!".into()));
            }
            let u: u32 = stuff[1].parse().unwrap();
            let v: u32 = stuff[2].parse().unwrap();
            let u_idx = *nodes.get(u as usize - 1).unwrap();
            let v_ind = *nodes.get(v as usize - 1).unwrap();
            graph.add_edge(u_idx, v_ind, ());
            continue;
        }
        // p edge 200 13089
        if line.starts_with('p') {
            let stuff: Vec<&str> = line.split_ascii_whitespace().collect();
            let node_count: usize = stuff[2].parse().unwrap();
            nodes = (0..node_count).map(|_| graph.add_node(())).collect();
        }
    }
    Ok(graph)
}
