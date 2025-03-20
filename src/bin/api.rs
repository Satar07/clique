use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use clique::max_clique::find_max_cliques;
use petgraph::graph::{NodeIndex, UnGraph};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
struct GraphRequest {
    edges: Vec<(usize, usize)>,
}

#[derive(Serialize)]
struct GraphResponse {
    max_clique: Vec<usize>,
}

async fn find_max_clique(
    data: web::Json<GraphRequest>,
    graph: web::Data<Arc<Mutex<UnGraph<(), ()>>>>,
) -> impl Responder {
    let mut graph = graph.lock().await;
    graph.clear();

    // 首先找出所有节点并排序
    let mut nodes: Vec<usize> = data
        .edges
        .iter()
        .flat_map(|(u, v)| [*u, *v])
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    nodes.sort();

    // 创建节点索引映射
    let node_to_idx: std::collections::HashMap<usize, NodeIndex> = nodes
        .iter()
        .enumerate()
        .map(|(idx, &node)| (node, NodeIndex::new(idx)))
        .collect();

    // 添加所有节点
    for _ in &nodes {
        graph.add_node(());
    }

    // 添加边到图中
    for (u, v) in &data.edges {
        let u_idx = node_to_idx[u];
        let v_idx = node_to_idx[v];
        graph.add_edge(u_idx, v_idx, ());
    }

    // 调用最大团算法
    let max_clique_indices = find_max_cliques(&graph);
    let max_clique: Vec<usize> = max_clique_indices
        .iter()
        .map(|idx| nodes[idx.index()])
        .collect();

    HttpResponse::Ok().json(GraphResponse { max_clique })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let graph: Arc<Mutex<UnGraph<(), ()>>> = Arc::new(Mutex::new(UnGraph::new_undirected()));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(graph.clone()))
            .route("/api/find-max-clique", web::post().to(find_max_clique))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
