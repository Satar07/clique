#[cfg(test)]
mod tests {
    use clique::graph_reader::read_dimacs;

    #[test]
    fn test_read(){
        // p edge 200 13089
        let graph = read_dimacs("data/brock200_4.clq").unwrap();
        assert_eq!(graph.node_count(),200);
        assert_eq!(graph.edge_count(),13089);
    }

    #[test]
    fn test_brock200_4() {
        let graph = read_dimacs("data/brock200_4.clq").unwrap();
        // let clique = bron_kerbosch(&graph);
        // assert!(clique.len(), 17);
    }
}
