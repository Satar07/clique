use petgraph::graph::{NodeIndex, UnGraph};
use rand::prelude::*;
use std::collections::HashSet;

#[derive(Clone)]
struct GAConfig {
    population_size: usize,
    mutation_rate: f64,
    max_generations: usize,
    shuffle_tolerance: usize,
}

impl Default for GAConfig {
    fn default() -> Self {
        GAConfig {
            population_size: 100,
            mutation_rate: 0.1,
            max_generations: 100,
            shuffle_tolerance: 10,
        }
    }
}

pub fn find_max_cliques_with_ga(graph: &UnGraph<(), ()>) -> Vec<NodeIndex> {
    // 遗传算法的具体实现
    let config = GAConfig::default();
    let mut ga = GeneticAlgorithm::new(graph, config);
    for _ in 0..ga.config.max_generations {
        ga.evolve();
    }
    ga.best_clique()
}

struct Clique<'a> {
    clique: HashSet<NodeIndex>, // 已有最大团
    pa: HashSet<NodeIndex>,     // 候选集 Possible Additions
    graph: &'a UnGraph<(), ()>,
}

impl<'a> Clique<'a> {
    fn new(graph: &'a UnGraph<(), ()>, start: NodeIndex) -> Self {
        let mut clique = HashSet::new();
        clique.insert(start);
        let mut pa = HashSet::new();
        for neighbors in graph.neighbors(start) {
            pa.insert(neighbors);
        }
        Clique { clique, pa, graph }
    }

    // 添加一个节点到最大团中
    // 注意！！这里不检查加入是否合法！！！
    fn add_vertex(&mut self, node: NodeIndex) {
        if !self.clique.contains(&node) {
            self.clique.insert(node);
            self.pa.retain(|&n| self.graph.contains_edge(n, node)); // 只保留和添加的相连的为候选
        }
    }

    // 从最大团中移除一个节点
    fn remove_vertex(&mut self, node: &NodeIndex) {
        // if remove success
        if self.clique.remove(node) {
            // 对于不在最大团里的节点，如果和现有最大团中的都相连，就加入pa
            self.pa = self
                .graph
                .node_indices()
                .filter(|&n| !self.clique.contains(&n))
                .filter(|&n| self.clique.iter().all(|&c| self.graph.contains_edge(n, c)))
                .collect();
        }
    }

    // 计算一个节点在子图中的度
    fn degree_in_subgraph(&self, node: NodeIndex, subgraph: &HashSet<NodeIndex>) -> usize {
        subgraph
            .iter()
            .filter(|&&n| self.graph.contains_edge(node, n))
            .count()
    }

    // 对当前最大团进行局部改进（随机移除两个节点，然后按度数排序重新加入）
    fn local_improvement(&mut self, iteration: usize, rng: &mut impl Rng) {
        let mut best = self.clone();
        for _ in 0..iteration {
            let mut temp = self.clone();
            let nodes: Vec<_> = temp.clique.iter().cloned().collect();

            if nodes.len() > 1 {
                let (n1, n2) = pick_two(&nodes, rng);
                temp.remove_vertex(n1);
                temp.remove_vertex(n2);
                temp.greedy_expand_in_pa();
            }

            if temp.clique.len() > best.clique.len() {
                best = temp.clone();
            }
        }

        *self = best;
    }

    // 简单的根据pa中子图的度数顺序尽可能添加
    fn greedy_expand_in_pa(&mut self) {
        let mut sorted: Vec<_> = self
            .pa
            .iter()
            .map(|&n| (n, self.degree_in_subgraph(n, &self.pa)))
            .collect();
        sorted.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        for (node, _) in sorted {
            if self.pa.is_empty() {
                break;
            }
            if self.pa.contains(&node) {
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
            graph: self.graph,
        }
    }
}

struct GeneticAlgorithm<'a> {
    population: Vec<Clique<'a>>,
    graph: &'a UnGraph<(), ()>,
    config: GAConfig,
    best_clique: HashSet<NodeIndex>,
    stagnation_counter: usize,
}

impl<'a> GeneticAlgorithm<'a> {
    fn new(graph: &'a UnGraph<(), ()>, config: GAConfig) -> Self {
        let mut rng = rand::rng();
        let mut population = Vec::with_capacity(config.population_size);

        // 初始种群生成
        // 随机生成贪心极大团个体
        let nodes: Vec<_> = graph.node_indices().collect();
        for _ in 0..config.population_size - 1 {
            let start = *nodes.choose(&mut rng).unwrap();
            let mut clique = Clique::new(graph, start);
            clique.greedy_expand_in_pa();
            population.push(clique);
        }

        // 添加度数最高节点的个体（天选之子）
        let max_degree_node = graph
            .node_indices()
            .max_by_key(|&n| graph.neighbors(n).count())
            .unwrap();
        let mut nb_clique = Clique::new(graph, max_degree_node);
        nb_clique.greedy_expand_in_pa();
        population.push(nb_clique);

        GeneticAlgorithm {
            population,
            graph,
            config,
            best_clique: HashSet::new(),
            stagnation_counter: 0,
        }
    }

    // main function
    fn evolve(&mut self) {
        let mut rng = rand::rng();
        let mut new_population = Vec::with_capacity(self.config.population_size);

        // 精英保留
        self.population
            .sort_unstable_by(|a, b| b.clique.len().cmp(&a.clique.len()));
        let now_best = self.population[0].clone();
        new_population.push(now_best);

        if self.population[0].clique.len() > self.best_clique.len() {
            self.best_clique = self.population[0].clique.clone();
            self.stagnation_counter = 0;
        } else {
            self.stagnation_counter += 1;
        }

        // 停滞处理
        if self.stagnation_counter >= self.config.shuffle_tolerance {
            // 练废了，重开罢
            *self = Self::new(self.graph, self.config.clone());
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

            child.local_improvement(10, &mut rng);
            new_population.push(child);
        }
        self.population = new_population;
    }

    fn crossover(&self, p1: &Clique, p2: &Clique) -> Clique<'a> {
        // 交集交叉
        let common_nodes: HashSet<_> = p1.clique.intersection(&p2.clique).copied().collect();
        if !common_nodes.is_empty() {
            let mut child = Clique::new(self.graph, *common_nodes.iter().next().unwrap());
            for &node in &common_nodes {
                if child.pa.contains(&node) {
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
        let all_nodes: HashSet<_> = p1.clique.union(&p2.clique).copied().collect();
        let mut sorted_nodes: Vec<_> = all_nodes
            .iter()
            .map(|&n| (n, self.graph.neighbors(n).count()))
            .collect();
        sorted_nodes.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        let mut child = Clique::new(self.graph, sorted_nodes[0].0);
        for (node, _) in &sorted_nodes[1..] {
            if child.pa.contains(node) {
                child.add_vertex(*node);
            }
        }
        child.greedy_expand_in_pa();
        child
    }

    fn mutate(&self, clique: &mut Clique, rng: &mut impl Rng) {
        // 删掉一个先
        let nodes: Vec<_> = clique.clique.iter().copied().collect();
        let idx = rng.random_range(0..nodes.len());
        clique.remove_vertex(&nodes[idx]);

        // 一半的几率贪心扩展
        if rng.random_bool(0.5) {
            clique.greedy_expand_in_pa();
            return;
        }

        // 另一半的几率随机拓展
        while !clique.pa.is_empty(){
            let node = clique.pa.iter().choose(rng).expect("should have at least one");
            clique.add_vertex(*node);
        }
    }

    fn best_clique(&mut self) -> Vec<NodeIndex> {
        self.best_clique.iter().copied().collect()
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
