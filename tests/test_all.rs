

#[cfg(test)]
mod tests {
    use clique::graph_reader::read_dimacs;
    use clique::max_clique::find_max_cliques_with_bk;
    use rstest::rstest;
    use std::time::Instant;

    #[rstest]
    #[case::brock200_2("brock200_2.clq", 12)]
    #[case::brock200_4("brock200_4.clq", 17)]
    #[case::brock400_2("brock400_2.clq", 29)]
    #[case::brock400_4("brock400_4.clq", 33)]
    #[case::brock800_2("brock800_2.clq", 24)]
    #[case::brock800_4("brock800_4.clq", 26)]
    #[case::C125_9("C125.9.clq", 34)]
    #[case::C250_9("C250.9.clq", 44)]
    #[case::C500_9("C500.9.clq", 57)]
    #[case::C1000_9("C1000.9.clq", 68)]
    #[case::C2000_9("C2000.9.clq", 80)]
    #[case::DSJC1000_5("DSJC1000_5.clq", 15)]
    #[case::DSJC500_5("DSJC500_5.clq", 13)]
    #[case::C2000_5("C2000.5.clq", 16)]
    #[case::C4000_5("C4000.5.clq", 18)]
    #[case::MANN_a27("MANN_a27.clq", 126)]
    #[case::MANN_a45("MANN_a45.clq", 345)]
    #[case::MANN_a81("MANN_a81.clq", 1100)]
    #[case::gen200_p0_9_44("gen200_p0.9_44.clq", 44)]
    #[case::gen200_p0_9_55("gen200_p0.9_55.clq", 55)]
    #[case::gen400_p0_9_55("gen400_p0.9_55.clq", 55)]
    #[case::gen400_p0_9_65("gen400_p0.9_65.clq", 65)]
    #[case::gen400_p0_9_75("gen400_p0.9_75.clq", 75)]
    #[case::hamming10_4("hamming10-4.clq", 40)]
    #[case::hamming8_4("hamming8-4.clq", 16)]
    #[case::keller4("keller4.clq", 11)]
    #[case::keller5("keller5.clq", 27)]
    #[case::keller6("keller6.clq", 59)]
    #[case::p_hat300_1("p_hat300-1.clq", 8)]
    #[case::p_hat300_2("p_hat300-2.clq", 25)]
    #[case::p_hat300_3("p_hat300-3.clq", 36)]
    #[case::p_hat700_1("p_hat700-1.clq", 11)]
    #[case::p_hat700_2("p_hat700-2.clq", 44)]
    #[case::p_hat700_3("p_hat700-3.clq", 62)]
    #[case::p_hat1500_1("p_hat1500-1.clq", 12)]
    #[case::p_hat1500_2("p_hat1500-2.clq", 65)]
    #[case::p_hat1500_3("p_hat1500-3.clq", 94)]
    #[tokio::test]
    async fn parallel_clique_test(
        #[case] filename: &str,
        #[case] expected_size: usize,
    ) {
        // 1. 构建文件路径
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let file_path = format!("{}/data/{}", manifest_dir, filename);
        
        // 2. 读取图数据
        let graph = read_dimacs(&file_path)
            .unwrap_or_else(|_| panic!("Failed to read {}", filename));

        // 3. 执行算法并计时
        let start = Instant::now();
        let clique = find_max_cliques_with_bk(&graph);
        let duration = start.elapsed();

        // 4. 断言结果
        assert_eq!(
            clique.len(),
            expected_size,
            "❌ {}: Expected {}, got {}",
            filename,
            expected_size,
            clique.len()
        );

        // 5. 输出性能信息
        println!(
            "✅ {}: Size {} in {:?} (Nodes: {}, Edges: {})",
            filename,
            expected_size,
            duration,
            graph.node_count(),
            graph.edge_count()
        );
    }
}