# Clique - 最大团算法实现

> 综合设计实验作品 2025.3

这是一个使用 Rust 实现的最大团算法项目，支持使用 Bron-Kerbosch Pivot 算法和遗传算法来求解无向图中的最大团问题，并具有可视化实例演示。

## 项目结构

```
.
├── src/                   # Rust源代码目录
│   ├── bin/               # 可执行文件目录
│   ├── ga.rs              # 遗传算法实现
│   ├── max_clique.rs      # 最大团算法核心实现
│   ├── graph_reader.rs    # 图数据读取模块
│   └── lib.rs             # 库入口文件
├── frontend/              # 前端界面
├── tests/                 # 测试文件
├── data/                  # 测试数据
└── Cargo.toml             # Rust项目配置文件
```

## 功能特点

- 支持多种最大团算法实现：
  - Bron-Kerbosch算法（适用于小规模图）
  - 遗传算法（适用于大规模图）
- 自动算法选择：根据图的大小和密度自动选择合适的算法
- 高性能实现：使用位运算和并行计算优化性能
- 提供 Web 界面可视化结果

## 编译项目

1. 确保已安装 Rust 工具链：
2. 克隆项目并编译：
```bash
git clone https://github.com/yourusername/clique.git
cd clique
cargo build --release
```

## 使用方法

### 可视化服务

1. 确保已安装 Node.js 和 npm：
2. 安装前端依赖：
```bash
cd frontend
npm install
```
3. 启动后端 API 服务：
```bash
cd ..
cargo run --release --bin api
```

4. 启动前端开发服务器：
```bash
cd frontend
npm start
```

5. 访问Web界面：
打开浏览器访问 http://localhost:3000

### 作为库使用
```rust
use clique::find_max_cliques;
use petgraph::graph::UnGraph;

let mut graph = UnGraph::new_undirected();
// 添加节点和边
let result = find_max_cliques(&graph);
```

### 测试最大团算法
```bash
cargo test --test test_all -- --nocapture
```

或者通过 nextest 测试（需 cargo 安装 nextest）

```bash
cargo nextest run 
```

### 修改遗传算法参数

在 `src/ga.rs` 中修改，默认参数如下：

```rs
GAConfig {
   population_size: 15,
   max_generations: 300,
   shuffle_tolerance: 10,
   local_improvement_iter: 10,
}
```

## 算法原理

### Bron-Kerbosch算法

Bron-Kerbosch算法是一种用于寻找图中所有最大团的回溯算法。主要思想是：

1. 维护三个集合：
   - R：当前正在构建的团
   - P：可能加入团的候选节点
   - X：已经处理过的节点

2. 算法步骤：
   - 当P和X都为空时，R就是一个最大团
   - 选择枢轴节点来减少递归分支
   - 对每个候选节点进行递归搜索

这里使用带枢轴（Pivot）的剪枝：只需递归处理与枢轴 ​**不相邻** 的节点，跳过其邻居。
我们可以证明下面的命题：

> **假设存在一个极大团，既不包含 u，也不包含任何与 u 不相邻的节点**

既然所有极大团必须满足上述条件，我们可以分两种情况处理：

1. ​**包含枢轴 u 的极大团**：在递归的其他分支中，当 u 被选中时会处理。
2. ​**不包含 u 但包含其非邻居的极大团**：在当前分支中处理与非邻居节点相关的路径。

因此，​**当前分支只需处理与 u 不相邻的节点**，而跳过 u 的邻居（因为它们会被其他分支覆盖）。

伪代码：

```python
def BronKerbosch_Pivot(R, P, X):
    if P 和 X 均为空:
        输出 R 作为一个极大团
        return
    u = 从 P ∪ X 中选择一个枢轴节点（通常选度数最高的）
    for v in P \ 邻居(u):  # 只处理与枢轴不相邻的节点
        BronKerbosch_Pivot(R ∪ {v}, P ∩ 邻居(v), X ∩ 邻居(v))
        P.remove(v)
        X.add(v)
```

### 遗传算法

对于大规模图，项目使用遗传算法来**近似求解**最大团问题：

主要参考原理来源如下，算法有部分改动
```
@misc{shah2020gclique,
  title={GCLIQUE: An Open Source Genetic Algorithm for the Maximum Clique Problem},
  author={Shah, Shalin},
  year={2020},
  doi={10.5281/zenodo.3829645}
}
```

#### 操作原语

- **greedy_expand_in_pa**：在候选集中贪心拓展当前最大团。这里的贪心指的是对在候选集子图中的度数排序，然后按度数尽可能的加入当前的最大团。
- **local_improvement**：对当前的最大团状态进行扰动，尝试移除几个节点然后再次拓展，保留其最大的结果。
- **pick_two**：在给定集合中挑选两个不同索引的对象，如果长度导致找不到则返回重复的两个。
- **crossover**：交配繁殖
   - 首先尝试交集，对交集进行贪心拓展
   - 对交集是空的情况，把范围先限制在父母的并集中，进行贪心拓展，最后如可能再进行对候选集中的贪心拓展
- **mutate**：进行移除部分节点，然后有一半几率进行贪心拓展，一半几率进行随机拓展

#### 进化过程

1. 初始化种群：随机选择节点进行 greedy_expand_in_pa
2. 停滞处理：如检测当前最大团的上界已经很久没有刷新了，就重新进行初始化种群
3. 存储当前最优解
4. 精英保留：对当前的精英（最大个体）进行保留，并进行 loacl_improvement
5. 后代生成：对上一代进行 pick_two 作为父母，进行 crossover
6. 变异：如果产生的后代变差，则尝试 mutate，然后进行 local_improvement
7. 迭代形成新的种群

## 前端界面

采用 d3 图形库展示图连接 https://github.com/d3/d3/blob/main/LICENSE

## 许可证

MIT License
