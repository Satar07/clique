use bitvec::prelude::*;
use petgraph::{
    graph::{NodeIndex, UnGraph},
    visit::EdgeRef,
};
use rand::prelude::*;
use std::ops::Not;

// TODO 多线程优化

#[derive(Clone)]
struct GAConfig {
    population_size: usize,
    mutation_rate: f64,
    max_generations: usize,
    shuffle_tolerance: usize,
    local_improvement_iter: usize,
}

impl Default for GAConfig {
    fn default() -> Self {
        GAConfig {
            population_size: 120,
            mutation_rate: 0.1,
            max_generations: 200,
            shuffle_tolerance: 10,
            local_improvement_iter: 10,
        }
    }
}

pub fn find_max_cliques_with_ga(graph: &UnGraph<(), ()>) -> Vec<NodeIndex> {
    // 构建邻接矩阵
    let node_count = graph.node_count();
    let mut adj_matrix = vec![bitvec![0;node_count]; node_count];
    for edge in graph.edge_references() {
        let (a, b) = (edge.source().index(), edge.target().index());
        adj_matrix[a].set(b, true);
        adj_matrix[b].set(a, true);
    }

    // 遗传算法的具体实现
    let config = GAConfig::default();
    let mut ga = GeneticAlgorithm::new(&adj_matrix, config);
    for _ in 0..ga.config.max_generations {
        ga.evolve();
    }

    // 映射回去
    ga.best_clique().iter_ones().map(NodeIndex::new).collect()
}

struct Clique<'a> {
    clique: BitVec, // 已有最大团
    pa: BitVec,     // 候选集 Possible Additions
    adj_matrix: &'a [BitVec],
    node_count: usize,
}

impl<'a> Clique<'a> {
    fn new(adj_matrix: &'a [BitVec], start: usize) -> Self {
        let node_count = adj_matrix.len();
        let mut clique = bitvec![0;node_count];
        clique.set(start, true);
        let mut pa = adj_matrix[start].clone();
        pa.set(start, false);
        Clique {
            clique,
            pa,
            adj_matrix,
            node_count,
        }
    }

    // 添加一个节点到最大团中
    // 注意！！这里不检查加入是否合法！！！
    fn add_vertex(&mut self, node: usize) {
        if self.clique[node] == false {
            self.clique.set(node, true);
            self.pa &= &self.adj_matrix[node];
            self.pa.set(node, false);
        }
    }

    // 从最大团中移除一个节点
    // 复杂度较高，需要重新计算一次产生新的pa
    fn remove_vertex(&mut self, node: usize) {
        if !self.clique[node] {
            return;
        }

        // 保存剩余团的位向量
        let mut remaining_clique = self.clique.clone();
        remaining_clique.set(node, false);

        // Step 1: 恢复被移除节点排除的候选节点
        self.pa |= &self.adj_matrix[node];

        // Step 2: 排除团中已有的节点
        self.pa &= &remaining_clique.clone().not();

        // Step 3: 过滤出与剩余团全连接的节点
        for clique_node in remaining_clique.iter_ones() {
            self.pa &= &self.adj_matrix[clique_node];
        }

        // 更新团状态
        self.clique = remaining_clique;
    }

    // 计算一个节点在子图中的度
    fn degree_in_subgraph(&self, node: usize, subgraph: &BitSlice) -> usize {
        (self.adj_matrix[node].clone() & subgraph).count_ones()
    }

    // TODO 询问此处是否出错
    // 对当前最大团进行局部改进（随机移除两个节点，然后按度数排序重新加入）
    fn local_improvement(&mut self, iteration: usize, rng: &mut impl Rng) {
        let mut best = self.clone();
        for _ in 0..iteration {
            let mut temp = self.clone();
            let nodes: Vec<_> = temp.clique.iter_ones().collect();

            if nodes.len() > 1 {
                let (n1, n2) = pick_two(&nodes, rng);
                temp.remove_vertex(*n1);
                temp.remove_vertex(*n2);
                temp.greedy_expand_in_pa();
            }

            if temp.clique.count_ones() > best.clique.count_ones() {
                best = temp;
            }
        }

        *self = best;
    }

    // 简单的根据pa中子图的度数顺序尽可能添加
    fn greedy_expand_in_pa(&mut self) {
        let pa_nodes: BitVec = self.pa.clone(); // 缓存当前PA
        let mut sorted: Vec<_> = self
            .pa
            .iter_ones()
            .map(|n| {
                // 使用 degree_in_subgraph 函数，避免重复计算
                (n, self.degree_in_subgraph(n, &pa_nodes))
            })
            .collect();

        sorted.sort_unstable_by_key(|&(_, deg)| std::cmp::Reverse(deg));

        for (node, _) in sorted {
            if self.pa[node] {
                // 检查node是否还在PA中
                self.add_vertex(node);
            }
        }
    }
}

impl Clone for Clique<'_> {
    fn clone(&self) -> Self {
        Clique {
            clique: self.clique.clone(),
            pa: self.pa.clone(),
            adj_matrix: self.adj_matrix, // is ref
            node_count: self.node_count,
        }
    }
}

struct GeneticAlgorithm<'a> {
    population: Vec<Clique<'a>>,
    adj_matrix: &'a [BitVec],
    config: GAConfig,
    best_clique: BitVec,
    stagnation_counter: usize,
}

impl<'a> GeneticAlgorithm<'a> {
    fn new(adj_matrix: &'a [BitVec], config: GAConfig) -> Self {
        let node_count = adj_matrix.len();
        let mut rng = rand::rng();
        let mut population = Vec::with_capacity(config.population_size);

        // 初始种群生成
        // 随机生成贪心极大团个体
        for _ in 0..config.population_size - 1 {
            let start = rng.random_range(0..node_count);
            let mut clique = Clique::new(adj_matrix, start);
            clique.greedy_expand_in_pa();
            population.push(clique);
        }

        // 添加度数最高节点的个体（天选之子）
        let max_degree_node = adj_matrix
            .iter()
            .enumerate()
            .max_by_key(|(_, vec)| vec.count_ones())
            .map(|(i, _)| i)
            .unwrap();

        let mut nb_clique = Clique::new(adj_matrix, max_degree_node);
        nb_clique.greedy_expand_in_pa();
        population.push(nb_clique);

        GeneticAlgorithm {
            population,
            adj_matrix,
            config,
            best_clique: bitvec![0;node_count],
            stagnation_counter: 0,
        }
    }

    // main function
    fn evolve(&mut self) {
        let mut rng = rand::rng();
        let mut new_population = Vec::with_capacity(self.config.population_size);

        // 精英保留(只需要找到最大值即可，无需排序)
        let now_best = self
            .population
            .iter()
            .max_by_key(|p| p.clique.count_ones())
            .unwrap()
            .clone();
        new_population.push(now_best.clone());

        if now_best.clique.count_ones() > self.best_clique.count_ones() {
            self.best_clique = now_best.clique.clone();
            self.stagnation_counter = 0;
        } else {
            self.stagnation_counter += 1;
        }

        // 停滞处理
        if self.stagnation_counter >= self.config.shuffle_tolerance {
            // 练废了，重开罢
            *self = Self::new(self.adj_matrix, self.config.clone());
            return;
        }

        // 生成后代
        while new_population.len() < self.config.population_size {
            // 直接选两个不一样的
            let (parent1, parent2) = pick_two(&self.population, &mut rng);
            // 生育！
            let mut child = self.crossover(parent1, parent2);

            if rng.random_bool(self.config.mutation_rate) {
                self.mutate(&mut child, &mut rng);
            }

            child.local_improvement(self.config.local_improvement_iter, &mut rng);
            new_population.push(child);
        }
        self.population = new_population;
    }

    fn crossover(&self, p1: &Clique, p2: &Clique) -> Clique<'a> {
        // 交集交叉
        let common_nodes: BitVec = p1.clique.clone() & p2.clique.clone();
        if common_nodes.any() {
            let mut child = Clique::new(self.adj_matrix, common_nodes.iter_ones().next().unwrap());
            for node in common_nodes.iter_ones() {
                if child.pa[node] == true {
                    child.add_vertex(node); // 两个团的交一定是团
                }
            }
            child.greedy_expand_in_pa();
            return child;
        }

        // 交集是空的情况，使用贪心生成后代
        // 1. 取两个的并集
        // 2. 计算在这个子图中的度数排序，按这个顺序尽可能加入极大团
        // 3. 若pa还没是空的，继续贪婪生成极大团
        let subgraph = p1.clique.clone() | p2.clique.clone();
        let mut sorted_nodes: Vec<_> = subgraph
            .iter_ones()
            .map(|n| (n, (subgraph.clone() & &self.adj_matrix[n]).count_ones()))
            .collect();
        sorted_nodes.sort_unstable_by_key(|&(_, deg)| std::cmp::Reverse(deg));

        let mut child = Clique::new(self.adj_matrix, sorted_nodes[0].0);
        for (node, _) in &sorted_nodes[1..] {
            if child.pa[*node] {
                child.add_vertex(*node);
            }
        }
        child.greedy_expand_in_pa();
        child
    }

    fn mutate(&self, clique: &mut Clique, rng: &mut impl Rng) {
        if clique.clique.not_all() {
            return;
        }

        // 删掉一个先
        let nodes: Vec<_> = clique.clique.iter_ones().collect();
        let idx = rng.random_range(0..nodes.len());
        clique.remove_vertex(nodes[idx]);

        // 一半的几率贪心扩展
        if rng.random_bool(0.5) {
            clique.greedy_expand_in_pa();
            return;
        }

        // 另一半的几率随机拓展
        while clique.pa.any() {
            let chosen = clique
                .pa
                .iter_ones()
                .choose(rng)
                .expect("at least one to choose...");
            clique.add_vertex(chosen);
        }
    }

    fn best_clique(&mut self) -> &BitVec {
        &self.best_clique
    }
}

fn pick_two<'a, T>(vec: &'a [T], rng: &mut impl Rng) -> (&'a T, &'a T) {
    assert!(!vec.is_empty(), "Input Vec must not be empty.");

    // 尝试选择两个不同的索引
    let samples: Vec<_> = vec.choose_multiple(rng, 2).collect();

    if samples.len() == 2 {
        (samples[0], samples[1])
    } else {
        // 没办法了，只能同一个
        (&vec[0], &vec[0])
    }
}
