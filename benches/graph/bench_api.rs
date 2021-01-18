use btree_graph::error::Error;
use btree_graph::graph::*;
use criterion::{black_box, Criterion};

fn setup() -> Result<graph<String, usize>, Error> {
    let mut graph: graph<String, usize> = graph::new();
    graph.add_vertex(String::from("0"));
    graph.add_vertex(String::from("1"));
    graph.add_vertex(String::from("2"));
    graph.add_vertex(String::from("3"));
    graph.add_vertex(String::from("4"));
    graph.add_vertex(String::from("5"));
    graph.add_vertex(String::from("6"));
    graph.add_vertex(String::from("7"));
    graph.add_vertex(String::from("8"));
    graph.add_vertex(String::from("9"));

    graph.add_edge(String::from("0"), String::from("1"), 1)?;
    graph.add_edge(String::from("0"), String::from("2"), 2)?;
    graph.add_edge(String::from("0"), String::from("3"), 3)?;
    graph.add_edge(String::from("0"), String::from("4"), 4)?;
    graph.add_edge(String::from("0"), String::from("5"), 5)?;
    graph.add_edge(String::from("0"), String::from("6"), 6)?;
    graph.add_edge(String::from("0"), String::from("7"), 7)?;
    graph.add_edge(String::from("0"), String::from("8"), 8)?;
    graph.add_edge(String::from("0"), String::from("9"), 9)?;

    graph.add_edge(String::from("1"), String::from("0"), 1)?;
    graph.add_edge(String::from("1"), String::from("2"), 2)?;
    graph.add_edge(String::from("1"), String::from("3"), 3)?;
    graph.add_edge(String::from("1"), String::from("4"), 4)?;
    graph.add_edge(String::from("1"), String::from("5"), 5)?;
    graph.add_edge(String::from("1"), String::from("6"), 6)?;
    graph.add_edge(String::from("1"), String::from("7"), 7)?;
    graph.add_edge(String::from("1"), String::from("8"), 8)?;
    graph.add_edge(String::from("1"), String::from("9"), 9)?;

    graph.add_edge(String::from("2"), String::from("0"), 1)?;
    graph.add_edge(String::from("2"), String::from("1"), 2)?;
    graph.add_edge(String::from("2"), String::from("3"), 3)?;
    graph.add_edge(String::from("2"), String::from("4"), 4)?;
    graph.add_edge(String::from("2"), String::from("5"), 5)?;
    graph.add_edge(String::from("2"), String::from("6"), 6)?;
    graph.add_edge(String::from("2"), String::from("7"), 7)?;
    graph.add_edge(String::from("2"), String::from("8"), 8)?;
    graph.add_edge(String::from("2"), String::from("9"), 9)?;

    graph.add_edge(String::from("3"), String::from("0"), 1)?;
    graph.add_edge(String::from("3"), String::from("1"), 2)?;
    graph.add_edge(String::from("3"), String::from("2"), 3)?;
    graph.add_edge(String::from("3"), String::from("4"), 4)?;
    graph.add_edge(String::from("3"), String::from("5"), 5)?;
    graph.add_edge(String::from("3"), String::from("6"), 6)?;
    graph.add_edge(String::from("3"), String::from("7"), 7)?;
    graph.add_edge(String::from("3"), String::from("8"), 8)?;
    graph.add_edge(String::from("3"), String::from("9"), 9)?;

    graph.add_edge(String::from("4"), String::from("0"), 1)?;
    graph.add_edge(String::from("4"), String::from("1"), 2)?;
    graph.add_edge(String::from("4"), String::from("2"), 3)?;
    graph.add_edge(String::from("4"), String::from("3"), 4)?;
    graph.add_edge(String::from("4"), String::from("5"), 5)?;
    graph.add_edge(String::from("4"), String::from("6"), 6)?;
    graph.add_edge(String::from("4"), String::from("7"), 7)?;
    graph.add_edge(String::from("4"), String::from("8"), 8)?;
    graph.add_edge(String::from("4"), String::from("9"), 9)?;

    graph.add_edge(String::from("5"), String::from("0"), 1)?;
    graph.add_edge(String::from("5"), String::from("1"), 2)?;
    graph.add_edge(String::from("5"), String::from("2"), 3)?;
    graph.add_edge(String::from("5"), String::from("3"), 4)?;
    graph.add_edge(String::from("5"), String::from("4"), 5)?;
    graph.add_edge(String::from("5"), String::from("6"), 6)?;
    graph.add_edge(String::from("5"), String::from("7"), 7)?;
    graph.add_edge(String::from("5"), String::from("8"), 8)?;
    graph.add_edge(String::from("5"), String::from("9"), 9)?;

    graph.add_edge(String::from("6"), String::from("0"), 1)?;
    graph.add_edge(String::from("6"), String::from("1"), 2)?;
    graph.add_edge(String::from("6"), String::from("2"), 3)?;
    graph.add_edge(String::from("6"), String::from("3"), 4)?;
    graph.add_edge(String::from("6"), String::from("4"), 5)?;
    graph.add_edge(String::from("6"), String::from("5"), 6)?;
    graph.add_edge(String::from("6"), String::from("7"), 7)?;
    graph.add_edge(String::from("6"), String::from("8"), 8)?;
    graph.add_edge(String::from("6"), String::from("9"), 9)?;

    graph.add_edge(String::from("7"), String::from("0"), 1)?;
    graph.add_edge(String::from("7"), String::from("1"), 2)?;
    graph.add_edge(String::from("7"), String::from("2"), 3)?;
    graph.add_edge(String::from("7"), String::from("3"), 4)?;
    graph.add_edge(String::from("7"), String::from("4"), 5)?;
    graph.add_edge(String::from("7"), String::from("5"), 6)?;
    graph.add_edge(String::from("7"), String::from("6"), 7)?;
    graph.add_edge(String::from("7"), String::from("8"), 8)?;
    graph.add_edge(String::from("7"), String::from("9"), 9)?;

    graph.add_edge(String::from("8"), String::from("0"), 1)?;
    graph.add_edge(String::from("8"), String::from("1"), 2)?;
    graph.add_edge(String::from("8"), String::from("2"), 3)?;
    graph.add_edge(String::from("8"), String::from("3"), 4)?;
    graph.add_edge(String::from("8"), String::from("4"), 5)?;
    graph.add_edge(String::from("8"), String::from("5"), 6)?;
    graph.add_edge(String::from("8"), String::from("6"), 7)?;
    graph.add_edge(String::from("8"), String::from("7"), 8)?;
    graph.add_edge(String::from("8"), String::from("9"), 9)?;

    Ok(graph)
}

pub fn clone_benchmark(c: &mut Criterion) {
    let graph = setup().unwrap();
    c.bench_function("graph::graph clone", |b| {
        b.iter(|| black_box(graph.clone()))
    });
}

pub fn edges_benchmark(c: &mut Criterion) {
    let graph = setup().unwrap();
    c.bench_function("graph::api::Edges", |b| b.iter(|| black_box(graph.edges())));
}

pub fn vertices_benchmark(c: &mut Criterion) {
    let graph = setup().unwrap();
    c.bench_function("graph::api::Vertices", |b| {
        b.iter(|| black_box(graph.vertices()))
    });
}

pub fn add_vertex_benchmark(c: &mut Criterion) {
    let mut graph = setup().unwrap();
    c.bench_function("graph::api::AddVertex (vertex does not exist)", |b| {
        b.iter(|| black_box(graph.add_vertex(String::from("10"))))
    });

    c.bench_function("graph::api::AddVertex (vertex exists)", |b| {
        b.iter(|| black_box(graph.add_vertex(String::from("0"))))
    });
}

pub fn add_edge_benchmark(c: &mut Criterion) {
    let mut graph = setup().unwrap();
    c.bench_function("graph::api::AddEdge (edge does not exist)", |b| {
        b.iter(|| black_box(graph.add_edge(String::from("9"), String::from("0"), 1)))
    });

    c.bench_function("graph::api::AddEdge (edge exists)", |b| {
        b.iter(|| black_box(graph.add_edge(String::from("0"), String::from("1"), 1)))
    });
}

pub fn get_edge_value_benchmark(c: &mut Criterion) {
    let graph = setup().unwrap();
    c.bench_function("graph::api::GetEdgeValue (edge does not exist)", |b| {
        b.iter(|| black_box(graph.get_edge_value(10)))
    });

    c.bench_function("graph::api::GetEdgeValue (edge exists)", |b| {
        b.iter(|| black_box(graph.get_edge_value(1)))
    });
}

pub fn get_vertex_value_benchmark(c: &mut Criterion) {
    let graph = setup().unwrap();
    c.bench_function("graph::api::GetVertexValue (vertex does not exist)", |b| {
        b.iter(|| black_box(graph.get_vertex_value(String::from("10"))))
    });

    c.bench_function("graph::api::GetVertexValue (vertex exists)", |b| {
        b.iter(|| black_box(graph.get_vertex_value(String::from("0"))))
    });
}

pub fn remove_edge_benchmark(c: &mut Criterion) {
    let mut graph = setup().unwrap();
    c.bench_function("graph::api::RemoveEdge (edge does not exist)", |b| {
        b.iter(|| black_box(graph.remove_edge(10)))
    });

    c.bench_function("graph::api::RemoveEdge (edge exists)", |b| {
        b.iter(|| black_box(graph.remove_edge(1)))
    });
}

pub fn remove_vertex_benchmark(c: &mut Criterion) {
    let mut graph = setup().unwrap();
    c.bench_function("graph::api::RemoveVertex (vertex does not exist)", |b| {
        b.iter(|| black_box(graph.remove_vertex(String::from("10"))))
    });

    c.bench_function("graph::api::RemoveVertex (vertex exists)", |b| {
        b.iter(|| black_box(graph.remove_vertex(String::from("0"))))
    });
}

pub fn adjacent_benchmark(c: &mut Criterion) {
    let graph = setup().unwrap();
    c.bench_function("graph::api::Adjacent (vertices are not adjacent)", |b| {
        b.iter(|| black_box(graph.adjacent(String::from("9"), String::from("0"))))
    });

    c.bench_function("graph::api::Adjacent (vertices are adjacent)", |b| {
        b.iter(|| black_box(graph.adjacent(String::from("0"), String::from("1"))))
    });

    c.bench_function("graph::api::Adjacent (vertex does not exist)", |b| {
        b.iter(|| black_box(graph.adjacent(String::from("10"), String::from("1"))))
    });
}

pub fn connections_benchmark(c: &mut Criterion) {
    let graph = setup().unwrap();
    c.bench_function("graph::api::Connections (vertex does not exist)", |b| {
        b.iter(|| black_box(graph.connections(String::from("10"))))
    });

    c.bench_function("graph::api::Connections (vertex exists)", |b| {
        b.iter(|| black_box(graph.connections(String::from("0"))))
    });

    c.bench_function("graph::api::Connections (vertex exists)", |b| {
        b.iter(|| black_box(graph.connections(String::from("8"))))
    });

    c.bench_function("graph::api::Connections (vertex exists)", |b| {
        b.iter(|| black_box(graph.connections(String::from("9"))))
    });
}
