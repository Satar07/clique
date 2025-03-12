use std::collections::BTreeSet;

use petgraph::graph::{NodeIndex, UnGraph};

pub fn bron_kerbosch(graph: &UnGraph<(), ()>) -> Vec<NodeIndex> {
    let mut result = BTreeSet::new();
    bk_dfs(
        graph,
        &mut BTreeSet::new(),
        &mut graph.node_indices().collect(),
        &mut BTreeSet::new(),
        &mut result,
    );
    return result.into_iter().collect();
}

fn bk_dfs(
    graph: &UnGraph<(), ()>,
    current_clique: &mut BTreeSet<NodeIndex>,
    candidates: &mut BTreeSet<NodeIndex>,
    excluded: &mut BTreeSet<NodeIndex>,
    max_clique: &mut BTreeSet<NodeIndex>,
) {
    // until the C and EX is empty, that is when max_clique maybe is current_clique
    if candidates.is_empty() && excluded.is_empty(){
        if current_clique.len() > max_clique.len(){
            *max_clique = current_clique.clone();
        }
        return;
    }
    

}
