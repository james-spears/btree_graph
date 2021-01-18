use criterion::criterion_main;

mod graph;
use graph::*;

criterion_main!(graph_benches,);
