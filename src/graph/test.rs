#![cfg(test)]

mod unit_tests {
    use crate::error::Error;
    use crate::graph::*;
    use alloc::collections::{BTreeMap, BTreeSet};

    #[test]
    fn test_graph() {
        let graph: BTreeGraph<usize, usize> = BTreeGraph::new();
        assert_eq!(graph, BTreeGraph::new())
    }

    #[test]
    fn definition() {
        // Instantiate a graph using the new associated function.
        let graph: BTreeGraph<usize, usize> = BTreeGraph::new();
        let vertices: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
        let edges: BTreeMap<usize, (usize, usize)> = BTreeMap::new();

        // Check graph struct is generated.
        assert_eq!(graph, BTreeGraph { vertices, edges })

        // Test passed
    }

    #[test]
    fn new_and_default() {
        // Instantiate a graph using the implementation of default.
        let graph: BTreeGraph<usize, usize> = BTreeGraph::new();

        // Check graph struct is equivalent to the struct generated
        // with the new associated function.
        assert_eq!(graph, BTreeGraph::default())

        // Test passed
    }

    #[test]
    fn vertices() {
        // Ensure there is a getter method for the vertices.
        let graph: BTreeGraph<usize, usize> = BTreeGraph::new();
        let vertices: BTreeSet<&usize> = BTreeSet::new();
        assert_eq!(graph.vertices(), vertices)

        // Test passed.
    }

    #[test]
    fn edges() {
        // Ensure there is a getter method for the edge.
        let graph: BTreeGraph<usize, usize> = BTreeGraph::new();
        let edges: BTreeSet<&usize> = BTreeSet::new();
        assert_eq!(graph.edges(), edges)

        // Test passed.
    }

    #[test]
    fn add_vertex() {
        // Add three nodes.
        let mut graph: BTreeGraph<usize, usize> = BTreeGraph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(graph.vertices().len(), 3);

        // Verify nodes retain order on read.
        let mut exp_vertices: BTreeSet<&usize> = BTreeSet::new();
        exp_vertices.insert(&0);
        exp_vertices.insert(&1);
        exp_vertices.insert(&2);
        assert_eq!(graph.vertices(), exp_vertices)

        // Test passed.
    }

    #[test]
    fn add_edge() -> Result<(), Error> {
        // Add three nodes.
        let mut graph: BTreeGraph<usize, usize> = BTreeGraph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(graph.vertices().len(), 3);

        // Add an edge (0, 1) = 2 and (1, 2) = 3.
        graph.add_edge(0, 1, 2)?;
        graph.add_edge(1, 2, 3)?;

        // There should now be two edges.
        let mut exp_edges: BTreeSet<&usize> = BTreeSet::new();
        exp_edges.insert(&2);
        exp_edges.insert(&3);
        assert_eq!(graph.edges(), exp_edges);

        // The value of (0, 1) is indeed 2.
        let mut exp_edges_0: BTreeSet<usize> = BTreeSet::new();
        exp_edges_0.insert(2);
        assert_eq!(graph.get_vertex_value(0).unwrap(), &exp_edges_0);

        // The value of (1, 2) is indeed 3.
        let mut exp_edges_1: BTreeSet<usize> = BTreeSet::new();
        exp_edges_1.insert(3);
        assert_eq!(graph.get_vertex_value(1).unwrap(), &exp_edges_1);

        // If you attempt to add an edge to a vertex that does not
        // exist, then an error is raised.
        assert_eq!(
            graph.add_edge(0, 3, 0).unwrap_err(),
            Error::VertexDoesNotExist
        );
        assert_eq!(
            graph.add_edge(3, 0, 0).unwrap_err(),
            Error::VertexDoesNotExist
        );
        assert_eq!(
            graph.add_edge(1, 3, 0).unwrap_err(),
            Error::VertexDoesNotExist
        );
        assert_eq!(
            graph.add_edge(3, 1, 0).unwrap_err(),
            Error::VertexDoesNotExist
        );

        // Tests passed.
        Ok(())
    }

    #[test]
    fn remove_vertex() -> Result<(), Error> {
        // Add three nodes.
        let mut graph: BTreeGraph<usize, usize> = BTreeGraph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(graph.vertices().len(), 3);

        // Add an edge (0, 1) = 2 and (1, 2) = 3.
        graph.add_edge(0, 1, 2)?;
        graph.add_edge(1, 2, 3)?;
        // Verify there are two edges.
        assert_eq!(graph.edges().len(), 2);

        // Remove the first node.
        graph.remove_vertex(0)?;

        // Check there remain only two nodes.
        let mut exp_vertices: BTreeSet<&usize> = BTreeSet::new();
        exp_vertices.insert(&1);
        exp_vertices.insert(&2);
        assert_eq!(graph.vertices(), exp_vertices);

        // Verify there are no dangling edges.
        let mut exp_edges: BTreeSet<&usize> = BTreeSet::new();
        exp_edges.insert(&3);
        assert_eq!(graph.edges(), exp_edges);

        // Add three nodes.
        let mut graph: BTreeGraph<usize, usize> = BTreeGraph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(graph.vertices().len(), 3);

        // Add an edge (0, 1) = 2 and (1, 2) = 3.
        graph.add_edge(0, 1, 2)?;
        graph.add_edge(1, 2, 3)?;
        // Verify there are two edges.
        assert_eq!(graph.edges().len(), 2);

        // Remove the first node.
        graph.remove_vertex(1)?;

        // Check there remain only two nodes.
        let mut exp_vertices: BTreeSet<&usize> = BTreeSet::new();
        exp_vertices.insert(&0);
        exp_vertices.insert(&2);
        assert_eq!(graph.vertices(), exp_vertices);

        let exp_edges_0: BTreeSet<usize> = BTreeSet::new();
        assert_eq!(graph.get_vertex_value(0).unwrap(), &exp_edges_0);

        // Verify there are no edges at all.
        let exp_edges: BTreeSet<&usize> = BTreeSet::new();
        assert_eq!(graph.edges(), exp_edges);

        Ok(())

        // Test passed.
    }

    #[test]
    fn remove_edge() -> Result<(), Error> {
        // Add three nodes.
        let mut graph: BTreeGraph<usize, usize> = BTreeGraph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(graph.vertices().len(), 3);

        // Add an edge (0, 1) = 2 and (1, 2) = 3.
        graph.add_edge(0, 1, 2)?;
        graph.add_edge(1, 2, 3)?;
        // Verify there are two edges.
        assert_eq!(graph.edges().len(), 2);

        // Remove the first edge.
        graph.remove_edge(2)?;

        // Verify there are still three nodes.
        assert_eq!(graph.vertices().len(), 3);

        // Verify there remains only one edge;
        assert_eq!(graph.edges().len(), 1);
        // and that edge has a value (1, 2).
        // let mut exp_edge: Edge<usize> = Edge::new();
        // exp_edge.from = 1;
        // exp_edge.to = 2;
        assert_eq!(graph.get_edge_value(3).unwrap(), &(1, 2));

        // Test passed.
        Ok(())
    }

    #[test]
    fn get_edge_value() -> Result<(), Error> {
        // Add three nodes.
        let mut graph: BTreeGraph<usize, usize> = BTreeGraph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(graph.vertices().len(), 3);

        // Add an edge (0, 1) = 2 and (1, 2) = 3.
        graph.add_edge(0, 1, 2)?;
        graph.add_edge(1, 2, 3)?;
        // Verify there are two edges;
        assert_eq!(graph.edges().len(), 2);
        // and the first has a value (0, 1);
        // let mut exp_edge_0: Edge<usize> = Edge::new();
        // exp_edge_0.from = 0;
        // exp_edge_0.to = 1;
        assert_eq!(graph.get_edge_value(2).unwrap(), &(0, 1));
        // and the second has a value (1, 2).
        // let mut exp_edge_1: Edge<usize> = Edge::new();
        // exp_edge_1.from = 1;
        // exp_edge_1.to = 2;
        assert_eq!(graph.get_edge_value(3).unwrap(), &(1, 2));

        // Test passed.
        Ok(())
    }

    #[test]
    fn get_vertex_value() -> Result<(), Error> {
        // Add three nodes.
        let mut graph: BTreeGraph<usize, usize> = BTreeGraph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(graph.vertices().len(), 3);

        // Add an edge (0, 1) = 2 and (1, 2) = 3.
        graph.add_edge(0, 1, 2)?;
        graph.add_edge(1, 2, 3)?;
        // Check there are in fact two edges.
        assert_eq!(graph.edges().len(), 2);

        let mut exp_edges_0: BTreeSet<usize> = BTreeSet::new();
        exp_edges_0.insert(2);
        assert_eq!(graph.get_vertex_value(0).unwrap(), &exp_edges_0);

        let mut exp_edges_1: BTreeSet<usize> = BTreeSet::new();
        exp_edges_1.insert(3);
        assert_eq!(graph.get_vertex_value(1).unwrap(), &exp_edges_1);

        // Test passed.
        Ok(())
    }

    #[test]
    fn adjacent() -> Result<(), Error> {
        // Add three nodes.
        let mut graph: BTreeGraph<usize, usize> = BTreeGraph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(graph.vertices().len(), 3);

        // Add an edge (0, 1) = 2 and (1, 2) = 3.
        graph.add_edge(0, 1, 2)?;
        graph.add_edge(1, 2, 3)?;
        // Check there are in fact two edges.
        assert_eq!(graph.edges().len(), 2);

        // By definition vertices 0, and 1 are adjacent.
        assert!(graph.adjacent(0, 1)?);
        // By definition vertices 1, and 0 are not adjacent.
        assert!(!graph.adjacent(1, 0)?);
        // By definition vertices 1, and 2 are adjacent.
        assert!(graph.adjacent(1, 2)?);
        // By definition vertices 2, and 1 are not adjacent.
        assert!(!graph.adjacent(2, 1)?);

        // If we attempt to check adjacency on a node that does not exist,
        // an error will be raised.
        assert_eq!(graph.adjacent(0, 3).unwrap_err(), Error::VertexDoesNotExist);
        assert_eq!(graph.adjacent(3, 0).unwrap_err(), Error::VertexDoesNotExist);

        // Test passed.
        Ok(())
    }

    #[test]
    fn connections() -> Result<(), Error> {
        // Add three nodes.
        let mut graph: BTreeGraph<usize, usize> = BTreeGraph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(graph.vertices().len(), 3);

        // Add an edge (0, 1) = 2, (1, 2) = 3, and (0, 2) = 4.
        graph.add_edge(0, 1, 2)?;
        graph.add_edge(1, 2, 3)?;
        graph.add_edge(0, 2, 4)?;

        // Check there are in fact two edges.
        assert_eq!(graph.edges().len(), 3);

        // There should be, by definition, two nodes (1, and 2)
        // 'connected' to node 0 through edges 2, and 4;
        let mut exp_connections_0: BTreeSet<&usize> = BTreeSet::new();
        exp_connections_0.insert(&1);
        exp_connections_0.insert(&2);
        assert_eq!(graph.connections(0)?, exp_connections_0);

        // similarly node 1 is 'connected' to only node 2 through the edge 3.
        let mut exp_connections_1: BTreeSet<&usize> = BTreeSet::new();
        exp_connections_1.insert(&2);
        assert_eq!(graph.connections(1)?, exp_connections_1);

        // If we try to check connections on a node that does not exist,
        // an error will be raised.
        assert_eq!(graph.connections(3).unwrap_err(), Error::VertexDoesNotExist);

        // Test passed.
        Ok(())
    }
}
