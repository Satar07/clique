#[cfg(test)]
mod tests {
    use clique::graph_reader::read_dimacs;
    use clique::max_clique::find_max_cliques;

    #[test]
    fn test_read() {
        // p edge 200 13089
        let graph = read_dimacs("data/brock200_4.clq").unwrap();
        assert_eq!(graph.node_count(), 200);
        assert_eq!(graph.edge_count(), 13089);
    }

    #[test]
    fn test_small() {
        let graph = read_dimacs("data/small.clq").unwrap();
        let clique = find_max_cliques(&graph);
        assert_eq!(clique.len(), 3);
    }

    // #[test]
    // fn test_brock200_4() {
    //     let graph = read_dimacs("data/brock200_4.clq").unwrap();
    //     let clique = find_max_cliques(&graph);
    //     assert_eq!(clique.len(), 17);
    // }

    #[test]
    fn test_brock200_2() {
        let graph = read_dimacs("data/brock200_2.clq").unwrap();
        let clique = find_max_cliques(&graph);
        assert_eq!(clique.len(), 12);
    }
}
