use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashSet;

pub fn find_max_cliques(graph: &UnGraph<(), ()>) -> Vec<NodeIndex> {
    let mut max_clique = HashSet::new();
    let all_nodes: HashSet<usize> = graph.node_indices().map(|u| u.index()).collect();

    let node_count = graph.node_count();
    let mut neighbors = Vec::with_capacity(node_count);
    let mut degrees = Vec::with_capacity(node_count);
    for u in graph.node_indices() {
        let ns: HashSet<usize> = graph.neighbors(u).map(|v| v.index()).collect();
        let degree = ns.len();
        neighbors.push(ns);
        degrees.push(degree);
    }

    bron_kerbosch_pivot(
        &neighbors,
        &degrees,
        &mut HashSet::new(),
        &mut all_nodes.clone(),
        &mut HashSet::new(),
        &mut max_clique,
    );

    max_clique.into_iter().map(|u| NodeIndex::new(u)).collect()
}

fn bron_kerbosch_pivot(
    neighbors: &[HashSet<usize>],
    degrees: &[usize],
    current_clique: &mut HashSet<usize>,
    candidates: &mut HashSet<usize>,
    excluded: &mut HashSet<usize>,
    max_clique: &mut HashSet<usize>,
) {
    if current_clique.len() + candidates.len() <= max_clique.len() {
        return;
    }

    if candidates.is_empty() && excluded.is_empty() {
        if current_clique.len() > max_clique.len() {
            *max_clique = current_clique.clone();
        }
        return;
    }

    // 选择枢轴节点（度数最大）
    let pivot = candidates
        .iter()
        .chain(excluded.iter())
        .max_by_key(|&&u| degrees[u])
        .copied();

    let remaining = if let Some(p) = pivot {
        candidates.difference(&neighbors[p]).cloned().collect()
    } else {
        candidates.clone()
    };

    for u in remaining {
        let u_neighbors = &neighbors[u];
        let mut new_candidates = candidates.intersection(u_neighbors).cloned().collect();
        let mut new_excluded = excluded.intersection(u_neighbors).cloned().collect();

        current_clique.insert(u);
        bron_kerbosch_pivot(
            neighbors,
            degrees,
            current_clique,
            &mut new_candidates,
            &mut new_excluded,
            max_clique,
        );
        current_clique.remove(&u);
        candidates.remove(&u);
        excluded.insert(u);
    }
}
