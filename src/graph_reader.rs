use std::{fs::File, io::{BufRead, BufReader}};
use petgraph::graph::UnGraph;
use thiserror::Error;

#[derive(Error,Debug)]
pub enum GraphErr {
    #[error("Invalid DIMACS format:{0}")]
    ParseError(String),
}

pub fn read_dimacs(path:&str)-> Result<UnGraph<(),()>,GraphErr>{
    let file = File::open(path).map_err(|e| GraphErr::ParseError(e.to_string()))?;
    let reader = BufReader::new(file);
    let mut graph = UnGraph::new_undirected();
    let mut node = Vec::new();

    for line in reader.lines(){
        let line = line.map_err(|e| GraphErr::ParseError(e.to_string()))?;
        if line.starts_with('e'){
            let stuff:Vec<&str> = line.split_ascii_whitespace().collect();
            if stuff.len()!=3{
                return Err(GraphErr::ParseError("bad edge format!".into()));
            }
        }
    }
    todo!()
}