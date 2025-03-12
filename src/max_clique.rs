use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::{HashMap, HashSet};

pub fn bron_kerbosch(graph: &UnGraph<(), ()>) -> Vec<NodeIndex> {
    let mut max_clique = HashSet::new();
    let all_nodes: HashSet<_> = graph.node_indices().collect();

    // 预存每个节点的邻居集合和度数
    let (neighbors, degrees) = {
        let mut neighbors = HashMap::new();
        let mut degrees = HashMap::new();
        for u in graph.node_indices() {
            let ns: HashSet<_> = graph.neighbors(u).collect();
            let degree = ns.len();
            neighbors.insert(u, ns);
            degrees.insert(u, degree);
        }
        (neighbors, degrees)
    };

    bron_kerbosch_pivot(
        &neighbors,
        &degrees,
        &mut HashSet::new(),
        &mut all_nodes.clone(),
        &mut HashSet::new(),
        &mut max_clique,
    );

    max_clique.into_iter().collect()
}

fn bron_kerbosch_pivot(
    neighbors: &HashMap<NodeIndex, HashSet<NodeIndex>>,
    degrees: &HashMap<NodeIndex, usize>,
    current_clique: &mut HashSet<NodeIndex>,
    candidates: &mut HashSet<NodeIndex>,
    excluded: &mut HashSet<NodeIndex>,
    max_clique: &mut HashSet<NodeIndex>,
) {
    // 剪枝：当前团无法超越最大团
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
        .max_by_key(|&u| degrees[u])
        .copied();

    let remaining = if let Some(p) = pivot {
        // 候选集中排除p的邻居
        candidates
            .difference(neighbors.get(&p).unwrap())
            .cloned()
            .collect()
    } else {
        candidates.clone()
    };

    for u in remaining {
        let u_neighbors = neighbors.get(&u).unwrap();

        // 生成新的候选集和排除集
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
