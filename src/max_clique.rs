use std::{cmp::Reverse, collections::{HashMap, HashSet}};

use petgraph::graph::{NodeIndex, UnGraph};

pub fn bron_kerbosch(graph: &UnGraph<(), ()>) -> Vec<NodeIndex> {
    let mut result = HashSet::new();
    let neighbors_map = graph
        .node_indices()
        .map(|n| (n, graph.neighbors(n).collect()))
        .collect();
    bk_dfs(
        graph,
        &neighbors_map,
        &mut HashSet::new(),
        &mut graph.node_indices().collect(),
        &mut HashSet::new(),
        &mut result,
    );
    return result.into_iter().collect();
}

fn bk_dfs(
    graph: &UnGraph<(), ()>,
    neighbors_map:&HashMap<NodeIndex,HashSet<NodeIndex>>,
    current_clique: &mut HashSet<NodeIndex>,
    candidates: &mut HashSet<NodeIndex>,
    excluded: &mut HashSet<NodeIndex>,
    max_clique: &mut HashSet<NodeIndex>,
) {
    // until the Candidate and Excluded is empty, that is when Max_clique maybe is current_clique
    if candidates.is_empty() && excluded.is_empty() {
        if current_clique.len() > max_clique.len() {
            *max_clique = current_clique.clone();
        }
        return;
    }

    // pick one pivot that have most degree, it will easy to cut branch
    let pivot = candidates.iter()
        .chain(excluded.iter())
        .max_by_key(|&n| neighbors_map[n].len())
        .copied();

    // expand node that not neibour with pivot
    let remain;
    if let Some(p) = pivot {
        remain = candidates
            .difference(&neighbors_map[&p])
            .copied()
            .collect();
    } else {
        remain = candidates.clone();
    }

    // sort the remain so that it can find max at first as possible
    let mut remain:Vec<_> = remain.into_iter().collect();
    remain.sort_by_key(|&n| Reverse(neighbors_map[&n].len()));

    for v in remain {
        let neighbors = graph.neighbors(v).collect();
        let mut new_candidates: HashSet<NodeIndex> =
            candidates.intersection(&neighbors).cloned().collect();
        let mut new_excluded: HashSet<NodeIndex> =
            excluded.intersection(&neighbors).cloned().collect();
        current_clique.insert(v);
        bk_dfs(
            graph,
            neighbors_map,
            current_clique,
            &mut new_candidates,
            &mut new_excluded,
            max_clique,
        );
        current_clique.remove(&v);
        candidates.remove(&v);
        excluded.insert(v);
    }
}
