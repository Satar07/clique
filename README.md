# Clique - 最大团算法实现

这是一个使用Rust实现的最大团算法项目，支持使用Bron-Kerbosch Pivot算法和遗传算法来求解无向图中的最大团问题，并额外具有可视化实例演示。

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
- 提供Web界面可视化结果

## 安装

1. 确保已安装Rust工具链：
2. 克隆项目并编译：
```bash
git clone https://github.com/yourusername/clique.git
cd clique
cargo build --release
```

## 使用方法

### 可视化服务

1. 确保已安装Node.js和npm：
2. 安装前端依赖：
```bash
cd frontend
npm install
```
3. 启动后端API服务：
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

或者通过 nextest 测试（需要cargo安装nextest）

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

### 遗传算法

对于大规模图，项目使用遗传算法来近似求解最大团问题：

主要参考原理来源，有改动
```
@misc{shah2020gclique,
  title={GCLIQUE: An Open Source Genetic Algorithm for the Maximum Clique Problem},
  author={Shah, Shalin},
  year={2020},
  doi={10.5281/zenodo.3829645}
}
```

1. 主要操作：
   - 初始化：随机生成多个可行解
   - 选择：保留适应度高的个体
   - 交叉：合并两个父代解
   - 变异：随机改变部分节点状态
   - 局部优化：使用贪心策略改进解

2. 优化策略：
   - 使用位运算加速计算
   - 实现局部搜索改进解的质量
   - 处理停滞问题，适时重新初始化种群
   - 在生成子代的时候采用并行计算


## 许可证

MIT License
