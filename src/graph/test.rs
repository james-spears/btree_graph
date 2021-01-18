#![cfg(test)]

mod unit_tests {
    #[cfg(any(feature = "serde_json", feature = "serde_yaml", feature = "serde_cbor"))]
    use crate::encoding::*;
    use crate::error::Error;
    use crate::graph::*;
    use alloc::collections::{BTreeMap, BTreeSet};
    #[cfg(any(feature = "serde_json", feature = "serde_yaml", feature = "serde_cbor"))]
    use alloc::string::String;

    #[test]
    fn test_graph() {
        let graph: graph<usize, usize> = graph::new();
        assert_eq!(graph, graph::new())
    }

    #[test]
    fn new() {
        // Instantiate a graph using the new associated function.
        let graph: graph<usize, usize> = graph::new();
        let vertices: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
        let edges: BTreeMap<usize, (usize, usize)> = BTreeMap::new();

        // Check graph struct is generated.
        assert_eq!(graph, graph { vertices, edges })

        // Test passed
    }

    #[test]
    fn default() {
        // Instantiate a graph using the implementation of default.
        let graph: graph<usize, usize> = graph::new();

        // Check graph struct is equivalent to the struct generated
        // with the new associated function.
        assert_eq!(graph, graph::new())

        // Test passed
    }

    #[test]
    fn vertices() {
        // Ensure there is a getter method for the vertices.
        let graph: graph<usize, usize> = graph::new();
        let vertices: BTreeSet<&usize> = BTreeSet::new();
        assert_eq!(graph.vertices(), vertices)

        // Test passed.
    }

    #[test]
    fn edges() {
        // Ensure there is a getter method for the edge.
        let graph: graph<usize, usize> = graph::new();
        let edges: BTreeSet<&usize> = BTreeSet::new();
        assert_eq!(graph.edges(), edges)

        // Test passed.
    }

    #[test]
    fn add_vertex() {
        // Add three nodes.
        let mut graph: graph<usize, usize> = graph::new();
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
        let mut graph: graph<usize, usize> = graph::new();
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
        let mut graph: graph<usize, usize> = graph::new();
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
        graph.remove_vertex(0);

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
        let mut graph: graph<usize, usize> = graph::new();
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
        graph.remove_vertex(1);

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
        let mut graph: graph<usize, usize> = graph::new();
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
        graph.remove_edge(2);

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
        let mut graph: graph<usize, usize> = graph::new();
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
        let mut graph: graph<usize, usize> = graph::new();
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
        let mut graph: graph<usize, usize> = graph::new();
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
        let mut graph: graph<usize, usize> = graph::new();
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

    #[test]
    #[cfg(all(feature = "serde", feature = "serde_json"))]
    fn test_try_from_json() -> Result<(), Error> {
        // Add three nodes.
        let mut graph: graph<usize, usize> = graph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(graph.vertices().len(), 3);

        // Add an edge (0, 1) = 2, (1, 2) = 3, and (0, 2) = 4.
        graph.add_edge(0, 1, 2)?;
        graph.add_edge(1, 2, 3)?;
        graph.add_edge(0, 2, 4)?;

        let decoded_graph: graph<usize, usize> = graph::try_from_json(String::from("{\"vertices\":{\"0\":[2,4],\"1\":[3],\"2\":[]},\"edges\":{\"2\":[0,1],\"3\":[1,2],\"4\":[0,2]}}"))?;
        assert_eq!(decoded_graph, graph);

        // &String
        let decoded_graph: graph<usize, usize> = graph::try_from_json(&String::from("{\"vertices\":{\"0\":[2,4],\"1\":[3],\"2\":[]},\"edges\":{\"2\":[0,1],\"3\":[1,2],\"4\":[0,2]}}"))?;
        assert_eq!(decoded_graph, graph);

        // &Vec<u8>
        let graph: graph<usize, usize> = graph::new();
        let bytes: Vec<u8> = graph.clone().try_into_json()?;
        assert_eq!(graph::try_from_json(bytes)?, graph);

        // &Vec<u8>
        let graph: graph<usize, usize> = graph::new();
        let bytes: Vec<u8> = graph.clone().try_into_json()?;
        assert_eq!(graph::try_from_json(&bytes)?, graph);
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "serde", feature = "serde_json"))]
    fn test_try_into_json() -> Result<(), Error> {
        // Add three nodes.
        let mut graph: graph<usize, usize> = graph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(graph.vertices().len(), 3);

        // Add an edge (0, 1) = 2, (1, 2) = 3, and (0, 2) = 4.
        graph.add_edge(0, 1, 2)?;
        graph.add_edge(1, 2, 3)?;
        graph.add_edge(0, 2, 4)?;

        let graph_string: String = graph.clone().try_into_json()?;
        assert_eq!(graph_string, String::from("{\"vertices\":{\"0\":[2,4],\"1\":[3],\"2\":[]},\"edges\":{\"2\":[0,1],\"3\":[1,2],\"4\":[0,2]}}"));

        // Bytes
        let graph: graph<usize, usize> = graph::new();
        let bytes: Vec<u8> = graph.clone().try_into_json()?;
        assert_eq!(bytes.as_slice().len(), 26);
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "serde", feature = "serde_cbor"))]
    fn test_try_into_cbor() -> Result<(), Error> {
        // Add three nodes.
        let mut graph: graph<usize, usize> = graph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(graph.vertices().len(), 3);

        // Add an edge (0, 1) = 2, (1, 2) = 3, and (0, 2) = 4.
        graph.add_edge(0, 1, 2)?;
        graph.add_edge(1, 2, 3)?;
        graph.add_edge(0, 2, 4)?;

        let bytes: Vec<u8> = graph.clone().try_into_cbor()?;
        assert_eq!(bytes.as_slice().len(), 39);
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "serde", feature = "serde_cbor"))]
    fn test_try_from_cbor() -> Result<(), Error> {
        // Add three nodes.
        let mut graph: graph<usize, usize> = graph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(graph.vertices().len(), 3);

        // Add an edge (0, 1) = 2, (1, 2) = 3, and (0, 2) = 4.
        graph.add_edge(0, 1, 2)?;
        graph.add_edge(1, 2, 3)?;
        graph.add_edge(0, 2, 4)?;

        // Vec<u8>
        let bytes: Vec<u8> = graph.clone().try_into_cbor()?;
        assert_eq!(graph::try_from_cbor(bytes)?, graph);

        // &Vec<u8>
        let graph: graph<usize, usize> = graph::new();
        let bytes: Vec<u8> = graph.clone().try_into_cbor()?;
        assert_eq!(graph::try_from_cbor(&bytes)?, graph);
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "serde", feature = "serde_yaml"))]
    fn test_try_from_yaml() -> Result<(), Error> {
        // Add three nodes.
        let mut graph: graph<usize, usize> = graph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(graph.vertices().len(), 3);

        // Add an edge (0, 1) = 2, (1, 2) = 3, and (0, 2) = 4.
        graph.add_edge(0, 1, 2)?;
        graph.add_edge(1, 2, 3)?;
        graph.add_edge(0, 2, 4)?;

        // String
        let decoded_graph: graph<usize, usize> = graph::try_from_yaml(String::from("---\nvertices:\n  0:\n    - 2\n    - 4\n  1:\n    - 3\n  2: []\nedges:\n  2:\n    - 0\n    - 1\n  3:\n    - 1\n    - 2\n  4:\n    - 0\n    - 2"))?;
        assert_eq!(decoded_graph, graph);

        // &String
        let decoded_graph: graph<usize, usize> = graph::try_from_yaml(&String::from("---\nvertices:\n  0:\n    - 2\n    - 4\n  1:\n    - 3\n  2: []\nedges:\n  2:\n    - 0\n    - 1\n  3:\n    - 1\n    - 2\n  4:\n    - 0\n    - 2"))?;
        assert_eq!(decoded_graph, graph);

        // &Vec<u8>
        let graph: graph<usize, usize> = graph::new();
        let bytes: Vec<u8> = graph.clone().try_into_yaml()?;
        assert_eq!(graph::try_from_yaml(bytes)?, graph);

        // &Vec<u8>
        let graph: graph<usize, usize> = graph::new();
        let bytes: Vec<u8> = graph.clone().try_into_yaml()?;
        assert_eq!(graph::try_from_yaml(&bytes)?, graph);
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "serde", feature = "serde_yaml"))]
    fn test_try_into_yaml() -> Result<(), Error> {
        // Add three nodes.
        let mut graph: graph<usize, usize> = graph::new();
        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);

        // Check there is indeed three nodes.
        assert_eq!(graph.vertices().len(), 3);

        // Add an edge (0, 1) = 2, (1, 2) = 3, and (0, 2) = 4.
        graph.add_edge(0, 1, 2)?;
        graph.add_edge(1, 2, 3)?;
        graph.add_edge(0, 2, 4)?;

        // String
        let graph_string: String = graph.clone().try_into_yaml()?;
        assert_eq!(graph_string, String::from("---\nvertices:\n  0:\n    - 2\n    - 4\n  1:\n    - 3\n  2: []\nedges:\n  2:\n    - 0\n    - 1\n  3:\n    - 1\n    - 2\n  4:\n    - 0\n    - 2"));

        // Bytes
        let graph: graph<usize, usize> = graph::new();
        let bytes: Vec<u8> = graph.clone().try_into_yaml()?;
        assert_eq!(bytes.as_slice().len(), 26);
        Ok(())
    }
}
