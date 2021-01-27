use alloc::collections::BTreeSet;

/// `Vertices` returns the set of the vertices which comprise the graph.
///
/// # Example
///
/// ```
/// use btree_graph::{BTreeGraph, AddVertex, Vertices};
/// let mut graph: BTreeGraph<String, usize> = BTreeGraph::new();
///
/// assert_eq!(graph.vertices().len(), 0);
/// ```
pub trait Vertices<T>
where
    T: Ord,
{
    fn vertices(&self) -> BTreeSet<&T>;
}

/// `Edges` returns the set edges which comprise the graph.
///
/// # Example
///
/// ```
/// use btree_graph::{BTreeGraph, AddVertex, AddEdge, Edges};
/// let mut graph: BTreeGraph<String, usize> = BTreeGraph::new();
///
/// assert_eq!(graph.edges().len(), 0);
/// ```
pub trait Edges<T>
where
    T: Ord,
{
    fn edges(&self) -> BTreeSet<&T>;
}

/// `AddVertex` adds the vertex x, if it is not there.
///
/// # Example
///
/// ```
/// use btree_graph::{BTreeGraph, AddVertex, Vertices};
/// use std::collections::BTreeSet;
/// let mut graph: BTreeGraph<String, usize> = BTreeGraph::new();
/// let old_vertex_value: Option<BTreeSet<usize>> = graph.add_vertex(String::from("origin"));
///
/// assert!(old_vertex_value.is_none());
/// assert_eq!(graph.vertices().len(), 1)
/// ```
pub trait AddVertex<V, E>
where
    E: Ord,
{
    fn add_vertex(&mut self, x: V) -> Option<BTreeSet<E>>;
}

/// `AddEdge` add an edge from the vertex x to the vertex y, if it is not there.
///
/// # Example
///
/// ```
/// use btree_graph::{BTreeGraph, AddVertex, AddEdge, Edges};
/// let mut graph: BTreeGraph<String, usize> = BTreeGraph::new();
/// graph.add_vertex(String::from("origin"));
/// graph.add_vertex(String::from("destination"));
/// let old_edge_value: Option<(String, String)> = graph.add_edge(String::from("origin"), String::from("destination"), 10).unwrap();
///
/// assert!(old_edge_value.is_none());
/// assert_eq!(graph.edges().len(), 1)
/// ```
pub trait AddEdge<V, E> {
    type Error;
    fn add_edge(&mut self, x: V, y: V, e: E) -> Result<Option<(V, V)>, Self::Error>;
}

/// `GetEdgeValue` returns the value associated with the edge (x, y).
///
/// # Example
///
/// ```
/// use btree_graph::{BTreeGraph, AddVertex, AddEdge, GetEdgeValue};
/// let mut graph: BTreeGraph<String, usize> = BTreeGraph::new();
/// graph.add_vertex(String::from("origin"));
/// graph.add_vertex(String::from("destination"));
/// graph.add_edge(String::from("origin"), String::from("destination"), 10);
///
/// let edge_value: &(String, String) = graph.get_edge_value(10).unwrap();
/// assert_eq!(edge_value.0, String::from("origin"));
/// assert_eq!(edge_value.1, String::from("destination"));
/// ```
pub trait GetEdgeValue<V, E> {
    fn get_edge_value(&self, x: E) -> Option<&(V, V)>;
}

/// `GetVertexValue` returns the value associated with the vertex x.
///
/// # Example
///
/// ```
/// extern crate alloc;
/// use alloc::collections::btree_set::BTreeSet;
/// use btree_graph::{BTreeGraph, AddVertex, AddEdge, GetEdgeValue, GetVertexValue};
/// let mut graph: BTreeGraph<String, usize> = BTreeGraph::new();
/// graph.add_vertex(String::from("origin"));
/// graph.add_vertex(String::from("destination"));
/// graph.add_edge(String::from("origin"), String::from("destination"), 10);
///
/// let vertex_value: &BTreeSet<usize> = graph.get_vertex_value(String::from("origin")).unwrap();
/// assert!(vertex_value.contains(&10));
/// ```
pub trait GetVertexValue<V, E>
where
    E: Ord,
{
    fn get_vertex_value(&self, x: V) -> Option<&BTreeSet<E>>;
}

/// `RemoveEdge` removes the edge from the vertex x to the vertex y, if it is there. If the
/// edge does not exist, an error will be raised.
///
/// # Example
///
/// ```
/// use btree_graph::{BTreeGraph, AddVertex, AddEdge, Edges, RemoveEdge, GetVertexValue};
/// let mut graph: BTreeGraph<String, usize> = BTreeGraph::new();
/// graph.add_vertex(String::from("origin"));
/// graph.add_vertex(String::from("destination"));
/// graph.add_edge(String::from("origin"), String::from("destination"), 10);
///
/// assert_eq!(graph.edges().len(), 1);
///
/// let removed_edge: (String, String) = graph.remove_edge(10).unwrap();
/// assert_eq!(removed_edge.0, String::from("origin"));
/// assert_eq!(removed_edge.1, String::from("destination"));
/// assert_eq!(graph.edges().len(), 0);
///
/// // Note: deletion of edges cascade i.e. the edge is also deleted from any incident
/// // vertices' adjacency lists.
/// assert_eq!(graph.get_vertex_value(String::from("origin")).unwrap().len(), 0);
/// ```
pub trait RemoveEdge<V, E> {
    type Error;
    fn remove_edge(&mut self, x: E) -> Result<(V, V), Self::Error>;
}

/// `RemoveVertex` removes the vertex x, if it is there. If the vertex does not exist,
/// an error is raised.
///
/// # Example
///
/// ```
/// extern crate alloc;
/// use alloc::collections::btree_set::BTreeSet;
/// use btree_graph::{BTreeGraph, AddVertex, AddEdge, Edges, RemoveVertex, GetVertexValue, Vertices};
/// let mut graph: BTreeGraph<String, usize> = BTreeGraph::new();
/// graph.add_vertex(String::from("origin"));
/// graph.add_vertex(String::from("destination"));
/// graph.add_edge(String::from("origin"), String::from("destination"), 10);
///
///
/// let adjacent_edges_removed = graph.remove_vertex(String::from("destination")).unwrap();
/// let adjacent_edges_removed_vec: Vec<(usize, (String, String))> = adjacent_edges_removed.into_iter().collect();
///
/// assert_eq!(adjacent_edges_removed_vec[0], (10, (String::from("origin"), String::from("destination"))));
/// assert_eq!(graph.vertices().len(), 1);
///
/// // Note: removing a vertex will also cascade delete any incident edges, which will then
/// // cascade delete any edges from the origin existing vertices' adjacency list.
/// assert_eq!(graph.edges().len(), 0);
/// assert_eq!(graph.get_vertex_value(String::from("origin")).unwrap().len(), 0);
/// ```
pub trait RemoveVertex<V, E>
where
    E: Ord,
{
    type Error;
    fn remove_vertex(&mut self, x: V) -> Result<BTreeSet<(E, (V, V))>, Self::Error>;
}

/// `Adjacent` tests whether there is an edge from the vertex x to the vertex y.
/// An error is thrown if either x, or y do not exist. By definition of adjacent there
/// must exist an edge e, with value (x, y) in order for vertices x, and y to be
/// considered adjacent.
///
/// # Example
///
/// ```
/// use btree_graph::{BTreeGraph, AddVertex, AddEdge, Adjacent};
/// let mut graph: BTreeGraph<String, usize> = BTreeGraph::new();
/// graph.add_vertex(String::from("origin"));
/// graph.add_vertex(String::from("destination"));
/// graph.add_edge(String::from("origin"), String::from("destination"), 10);
///
/// assert!(graph.adjacent(String::from("origin"), String::from("destination")).unwrap());
/// // Note: the graph is directed, and the definition of adjacent
/// // can be phrased, if there exists a relationship from x to y. Therefore
/// // A and B adjacent does not imply B and A are adjacent.
/// assert!(!graph.adjacent(String::from("destination"), String::from("origin")).unwrap());
/// ```
pub trait Adjacent<T> {
    type Error;
    fn adjacent(&self, x: T, y: T) -> Result<bool, Self::Error>;
}

/// `Connections` lists all vertices y such that there is an edge from the vertex x to
/// the vertex y. An error is thrown if x does not exist.
///
/// # Example
///
/// ```
/// use btree_graph::{BTreeGraph, AddVertex, AddEdge, Connections};
/// let mut graph: BTreeGraph<String, usize> = BTreeGraph::new();
/// graph.add_vertex(String::from("origin"));
/// graph.add_vertex(String::from("destination"));
/// graph.add_edge(String::from("origin"), String::from("destination"), 10);
///
/// assert!(graph.connections(String::from("origin")).unwrap().contains(&String::from("destination")));
/// ```
pub trait Connections<T> {
    type Error;
    fn connections(&self, x: T) -> Result<BTreeSet<&T>, Self::Error>;
}
