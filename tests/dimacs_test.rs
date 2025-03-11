#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_brock200_4() {
        let graph = read_dimacs("data/brock200_4.clq").unwrap();
        let clique = bron_kerbosch(&graph);
        assert!(clique.len(), 17);
    }
}
