use petgraph::graph::{NodeIndex, UnGraph};

pub fn bron_kerbosch(graph: &UnGraph<(), ()>) -> Vec<NodeIndex> {
    let mut current_clique = Vec::new(); // Init as empty
    let mut candidate = graph.node_indices().collect(); // Init as all
    let mut excluded = Vec::new(); // Init as empty
    let mut max_clique = Vec::new(); // RESULT : Initialize as empty
    bk_dfs(
        graph,
        &mut current_clique,
        &mut candidate,
        &mut excluded,
        &mut max_clique,
    );
    max_clique
}

fn bk_dfs(
    graph: &UnGraph<(), ()>,
    current_clique: &mut Vec<NodeIndex>,
    candidate: &mut Vec<NodeIndex>,
    excluded: &mut Vec<NodeIndex>,
    max_clique: &mut Vec<NodeIndex>,
) {
    // until the C and EX is empty, that is when max_clique maybe is current_clique
    if candidate.is_empty() && excluded.is_empty() {
        // flush the maximun situation
        if max_clique.len() < current_clique.len() {
            *max_clique = current_clique.clone();
        }
        return;
    }

    // pick one point to explore
    // when the rank is bigger, it will more likely to cut branch
    let pivot = candidate.iter().max_by_key(|&n| graph.neighbors(*n).count());
    // TODO 完成dfs部分
}
