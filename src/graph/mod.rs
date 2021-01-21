mod api;
mod test;

use alloc::collections::{BTreeMap, BTreeSet};
use core::default::Default;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error::Error;
pub use api::*;

/// `BTreeGraph` is an implementation of a graph (abstract data structure)
/// which utilizes `BTreeMap` for the edge and vertex adjacency lists.
#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BTreeGraph<V, E>
where
    V: Ord,
    E: Ord,
{
    vertices: BTreeMap<V, BTreeSet<E>>,
    edges: BTreeMap<E, (V, V)>,
}

impl<V, E> BTreeGraph<V, E>
where
    V: Ord,
    E: Ord,
{
    pub fn new() -> Self {
        let vertices: BTreeMap<V, BTreeSet<E>> = BTreeMap::new();
        let edges: BTreeMap<E, (V, V)> = BTreeMap::new();
        BTreeGraph { vertices, edges }
    }
}

impl<V, E> Default for BTreeGraph<V, E>
where
    V: Ord,
    E: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<V, E> Vertices<V> for BTreeGraph<V, E>
where
    V: Ord,
    E: Ord,
{
    fn vertices(&self) -> BTreeSet<&V> {
        self.vertices.keys().collect()
    }
}

impl<V, E> Edges<E> for BTreeGraph<V, E>
where
    V: Ord,
    E: Ord,
{
    fn edges(&self) -> BTreeSet<&E> {
        self.edges.keys().collect()
    }
}

impl<V, E> AddVertex<V, E> for BTreeGraph<V, E>
where
    V: Ord,
    E: Ord,
{
    fn add_vertex(&mut self, x: V) -> Option<BTreeSet<E>> {
        self.vertices.insert(x, BTreeSet::new())
    }
}

/// When you add an edge, you should make sure that the x, and y vertices exist.
impl<V, E> AddEdge<V, E> for BTreeGraph<V, E>
where
    V: Ord + Clone,
    E: Ord + Clone,
{
    type Error = Error;
    fn add_edge(&mut self, x: V, y: V, e: E) -> Result<Option<(V, V)>, Self::Error> {
        if self.vertices.get(&y).is_some() {
            if let Some(edges) = self.vertices.get(&x) {
                let mut v: BTreeSet<E> = edges.iter().cloned().collect();
                v.insert(e.clone());
                self.vertices.insert(x.clone(), v);
                return Ok(self.edges.insert(e, (x, y)));
            }
        }
        Err(Error::VertexDoesNotExist)
    }
}

impl<V, E> GetEdgeValue<V, E> for BTreeGraph<V, E>
where
    V: Ord,
    E: Ord,
{
    fn get_edge_value(&self, e: E) -> Option<&(V, V)> {
        self.edges.get(&e)
    }
}

impl<V, E> GetVertexValue<V, E> for BTreeGraph<V, E>
where
    V: Ord,
    E: Ord,
{
    fn get_vertex_value(&self, v: V) -> Option<&BTreeSet<E>> {
        self.vertices.get(&v)
    }
}

/// When an edge is removed, you should find the incident vertex and ensure the edge
/// is removed from the vertex's adjacency list.
impl<V, E> RemoveEdge<V, E> for BTreeGraph<V, E>
where
    V: Ord + Clone,
    E: Ord + Clone,
{
    type Error = Error;
    fn remove_edge(&mut self, e: E) -> Result<(), Self::Error> {
        if let Some(edge) = self.edges.get(&e) {
            let v = edge.0.clone();
            if let Some(vertex) = self.vertices.get(&v) {
                let mut vertex = vertex.clone();
                vertex.remove(&e);
                self.vertices.insert(v, vertex);
            }
            self.edges.remove(&e);
            return Ok(());
        }
        Err(Error::EdgeDoesNotExist)
    }
}

/// When you remove a vertex, you should ensure there are no dangling edges.
impl<V, E> RemoveVertex<V, E> for BTreeGraph<V, E>
where
    V: Ord + Clone,
    E: Ord + Clone,
{
    type Error = Error;
    fn remove_vertex(&mut self, v: V) -> Result<(), Self::Error> {
        // When removing an vertex, of course, we should remove
        // all adjacent edges;
        if let Some(edges) = self.vertices.get(&v) {
            for edge in edges {
                self.edges.remove(&edge);
            }
            // in addition we should be checking for, and removing
            // any edges which point to the vertex pending removal.
            self.edges
                .clone()
                .into_iter()
                .filter(|e| -> bool { e.1 .1 == v })
                .try_for_each(|e| self.remove_edge(e.0))?;
            self.vertices.remove(&v);
            return Ok(());
        }
        Err(Error::VertexDoesNotExist)
    }
}

impl<V, E> Adjacent<V> for BTreeGraph<V, E>
where
    V: Ord,
    E: Ord,
{
    type Error = Error;
    fn adjacent(&self, x: V, y: V) -> Result<bool, Self::Error> {
        if self.vertices.get(&y).is_some() {
            if let Some(edges) = self.vertices.get(&x) {
                for edge in edges {
                    // We can assume an edge exists if it is found adjacent
                    // to some vertex.
                    let e = self.edges.get(&edge).unwrap();
                    if e.0 == x && e.1 == y {
                        return Ok(true);
                    }
                }
                return Ok(false);
            }
        }
        Err(Error::VertexDoesNotExist)
    }
}

impl<V, E> Connections<V> for BTreeGraph<V, E>
where
    V: Ord,
    E: Ord,
{
    type Error = Error;
    fn connections(&self, v: V) -> Result<BTreeSet<&V>, Self::Error> {
        match self.vertices.get(&v) {
            Some(vertex) => Ok(vertex
                .iter()
                .map(|edge| -> &V { &self.edges.get(&edge).unwrap().1 })
                .collect()),
            None => Err(Error::VertexDoesNotExist),
        }
    }
}
