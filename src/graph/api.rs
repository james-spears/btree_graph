use alloc::collections::BTreeSet;

/// `Vertices` returns the set of the vertices which comprise the graph.
pub trait Vertices<T>
where
    T: Ord,
{
    fn vertices(&self) -> BTreeSet<&T>;
}

/// `Edges` returns the set edges which comprise the graph.
pub trait Edges<T>
where
    T: Ord,
{
    fn edges(&self) -> BTreeSet<&T>;
}

/// `AddVertex` adds the vertex x, if it is not there.
pub trait AddVertex<V, E>
where
    E: Ord,
{
    fn add_vertex(&mut self, x: V) -> Option<BTreeSet<E>>;
}

/// `AddEdge` add an edge from the vertex x to the vertex y, if it is not there.
pub trait AddEdge<V, E> {
    type Error;
    fn add_edge(&mut self, x: V, y: V, e: E) -> Result<Option<(V, V)>, Self::Error>;
}

/// `GetEdgeValue` returns the value associated with the edge (x, y).
pub trait GetEdgeValue<V, E> {
    fn get_edge_value(&self, x: E) -> Option<&(V, V)>;
}

/// `GetVertexValue` returns the value associated with the vertex x.
pub trait GetVertexValue<V, E>
where
    E: Ord,
{
    fn get_vertex_value(&self, x: V) -> Option<&BTreeSet<E>>;
}

/// `RemoveEdge` removes the edge from the vertex x to the vertex y, if it is there.
pub trait RemoveEdge<V, E> {
    fn remove_edge(&mut self, x: E) -> Option<(V, V)>;
}

/// `RemoveVertex` removes the vertex x, if it is there.
pub trait RemoveVertex<V, E>
where
    E: Ord,
{
    fn remove_vertex(&mut self, x: V) -> Option<BTreeSet<E>>;
}

/// `Adjacent` tests whether there is an edge from the vertex x to the vertex y.
/// An error is thrown if either x, or y do not exist. By definition of adjacent there
/// must exist an edge e, with value (x, y) in order for vertices x, and y to be
/// considered adjacent.
pub trait Adjacent<T> {
    type Error;
    fn adjacent(&self, x: T, y: T) -> Result<bool, Self::Error>;
}

/// `Connections` lists all vertices y such that there is an edge from the vertex x to
/// the vertex y. An error is thrown if x does not exist.
pub trait Connections<T> {
    type Error;
    fn connections(&self, x: T) -> Result<BTreeSet<&T>, Self::Error>;
}
