use fixedbitset::FixedBitSet;
use petgraph::graph::{NodeIndex, UnGraph};

use crate::ga::find_max_cliques_with_ga;

pub fn find_max_cliques(graph: &UnGraph<(), ()>) -> Vec<NodeIndex> {
    let (n, e) = (graph.node_count(), graph.edge_count());
    let density = 2.0 * e as f64 / (n * (n - 1)) as f64;

    // // 对于小图，直接使用 bk
    // if n <= 50
    //     || (n <= 100 && density <= 0.9)
    //     || (n < 200 && density <= 0.8)
    //     || (n <= 500 && density <= 0.3)
    // {
    //     return find_max_cliques_with_bk(graph);
    // }

    // 大图使用遗传算法
    return find_max_cliques_with_ga(graph);
}

fn find_max_cliques_with_bk(graph: &UnGraph<(), ()>) -> Vec<NodeIndex> {
    let node_count = graph.node_count();

    // 1. 构建原始邻接表
    let mut neighbors = vec![FixedBitSet::with_capacity(node_count); node_count];
    for u in graph.node_indices() {
        let idx = u.index();
        for v in graph.neighbors(u) {
            neighbors[idx].insert(v.index());
        }
    }

    // 2. 创建排序映射
    let (sorted_nodes, old_to_new) = {
        let mut nodes: Vec<usize> = (0..node_count).collect();
        nodes.sort_unstable_by_key(|&u| -(neighbors[u].count_ones(..) as i32));

        // 创建旧索引到新索引的映射
        let mut mapping = vec![0; node_count];
        for (new_idx, &old_idx) in nodes.iter().enumerate() {
            mapping[old_idx] = new_idx;
        }
        (nodes, mapping)
    };

    // 3. 构建排序后的邻接表
    let sorted_neighbors: Vec<FixedBitSet> = sorted_nodes
        .iter()
        .map(|&old_idx| {
            neighbors[old_idx]
                .ones()
                .map(|old_nb| old_to_new[old_nb])
                .collect()
        })
        .collect();

    // 4. 初始化集合
    let mut max_clique = FixedBitSet::with_capacity(node_count);
    let mut candidates = FixedBitSet::from_iter(0..node_count);
    let mut excluded = FixedBitSet::with_capacity(node_count);

    bron_kerbosch_pivot(
        &sorted_neighbors,
        &mut FixedBitSet::with_capacity(node_count),
        &mut candidates,
        &mut excluded,
        &mut max_clique,
    );

    // 5. 转换结果
    max_clique
        .ones()
        .map(|sorted_idx| NodeIndex::new(sorted_nodes[sorted_idx]))
        .collect()
}

fn bron_kerbosch_pivot(
    neighbors: &[FixedBitSet],
    current_clique: &mut FixedBitSet,
    candidates: &mut FixedBitSet,
    excluded: &mut FixedBitSet,
    max_clique: &mut FixedBitSet,
) {
    // 预计算大小
    let current_size = current_clique.count_ones(..);
    let candidates_size = candidates.count_ones(..);

    // 剪枝条件
    if current_size + candidates_size <= max_clique.count_ones(..) {
        return;
    }

    // 终止条件
    if candidates.is_clear() {
        if excluded.is_clear() && current_size > max_clique.count_ones(..) {
            max_clique.clone_from(current_clique);
        }
        return;
    }

    // 选择枢轴
    let pivot = select_pivot(candidates, excluded, neighbors);

    // 生成remaining集合
    let mut remaining = if let Some(p) = pivot {
        let mut diff = candidates.clone();
        diff.difference_with(&neighbors[p]);
        diff
    } else {
        candidates.clone()
    };

    // 遍历所有候选节点
    while let Some(u) = remaining.ones().next() {
        // 生成新候选集
        let mut new_candidates = candidates.clone();
        new_candidates.intersect_with(&neighbors[u]);

        // 提前剪枝
        if current_size + 1 + new_candidates.count_ones(..) <= max_clique.count_ones(..) {
            candidates.remove(u);
            excluded.insert(u);
            remaining.remove(u);
            continue;
        }

        // 准备递归参数
        current_clique.insert(u);
        let mut new_excluded = excluded.clone();
        new_excluded.intersect_with(&neighbors[u]);

        // 递归调用（传递副本而非原始引用）
        bron_kerbosch_pivot(
            neighbors,
            current_clique,
            &mut new_candidates,
            &mut new_excluded,
            max_clique,
        );

        // 回溯
        current_clique.remove(u);
        candidates.remove(u);
        excluded.insert(u);
        remaining.remove(u);
    }
}

fn select_pivot(
    candidates: &FixedBitSet,
    excluded: &FixedBitSet,
    neighbors: &[FixedBitSet],
) -> Option<usize> {
    candidates.ones().chain(excluded.ones()).max_by_key(|&u| {
        let mut tmp = neighbors[u].clone();
        tmp.intersect_with(candidates);
        tmp.count_ones(..)
    })
}
