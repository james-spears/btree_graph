use criterion::criterion_group;

mod bench_api;
pub use bench_api::*;

criterion_group!(
    graph_benches,
    clone_benchmark,
    edges_benchmark,
    vertices_benchmark,
    add_vertex_benchmark,
    add_edge_benchmark,
    get_edge_value_benchmark,
    get_vertex_value_benchmark,
    remove_edge_benchmark,
    remove_vertex_benchmark,
    adjacent_benchmark,
    connections_benchmark
);
