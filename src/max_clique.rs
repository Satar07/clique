use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::BTreeSet;

/// 带记忆化的Bron-Kerbosch算法实现
pub fn bron_kerbosch<P>(graph: &UnGraph<P, ()>) -> Vec<NodeIndex> {
    let mut max_clique = BTreeSet::new();
    let all_nodes: BTreeSet<_> = graph.node_indices().collect();

    bron_kerbosch_pivot(
        graph,
        &mut BTreeSet::new(),
        &mut all_nodes.clone(),
        &mut BTreeSet::new(),
        &mut max_clique,
    );

    max_clique.into_iter().collect()
}

fn bron_kerbosch_pivot<P>(
    graph: &UnGraph<P, ()>,
    current_clique: &mut BTreeSet<NodeIndex>,
    candidates: &mut BTreeSet<NodeIndex>,
    excluded: &mut BTreeSet<NodeIndex>,
    max_clique: &mut BTreeSet<NodeIndex>,
) {
    if candidates.is_empty() && excluded.is_empty() {
        if current_clique.len() > max_clique.len() {
            *max_clique = current_clique.clone();
        }
        return;
    }

    // 选择枢轴节点优化（按度数排序）
    let pivot = candidates
        .iter()
        .chain(excluded.iter())
        .max_by_key(|&u| graph.neighbors(*u).count())
        .copied();

    let remaining;
    if let Some(p) = pivot {
        remaining = candidates
            .difference(&graph.neighbors(p).collect())
            .cloned()
            .collect();
    } else {
        remaining = candidates.clone();
    }

    for u in remaining {
        let neighbors: BTreeSet<_> = graph.neighbors(u).collect();

        let mut new_candidates = candidates.intersection(&neighbors).cloned().collect();
        let mut new_excluded = excluded.intersection(&neighbors).cloned().collect();

        current_clique.insert(u);
        bron_kerbosch_pivot(
            graph,
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
