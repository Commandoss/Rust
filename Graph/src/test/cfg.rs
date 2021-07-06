// extern crate Graph;

// use Graph::direct::libGraph;
// use std::collections::HashMap;
//
#[cfg(test)]
mod tests {
    use crate::direct::libGraph::{Graph, Node};

    #[test]
    fn create_vetrex() {
        Node::new_node(23, "Check create vertex!");
    }

    #[test]
    fn create_graph() {
        let mut check_graph: Graph<u32, &str> = Graph::new();
        check_graph.add_node(1, "one");
        check_graph.add_node(2, "two");
        check_graph.add_node(3, "three");
        check_graph.add_node(4, "four");
        check_graph.add_node(5, "five");
        check_graph.add_node(6, "six");
    }

    #[test]
    fn add_oriented_rib() {
        let mut check_graph: Graph<u32, &str> = Graph::new();
        check_graph.add_node(1, "one");
        check_graph.add_node(2, "two");

        check_graph.add_oriented_rib(1, 2, 23);
    }

    #[test]
    fn traversal() {
        let mut check_graph: Graph<u32, &str> = Graph::new();
        check_graph.add_node(1, "one");
        check_graph.add_node(2, "two");
        check_graph.add_node(3, "three");
        check_graph.add_node(4, "four");
        check_graph.add_node(5, "five");
        check_graph.add_node(6, "six");

        check_graph.add_oriented_rib(1, 1, 23);
        check_graph.add_oriented_rib(1, 2, 23);
        check_graph.add_oriented_rib(1, 3, 23);
        check_graph.add_oriented_rib(1, 4, 23);
        check_graph.add_oriented_rib(4, 5, 23);
        check_graph.add_oriented_rib(5, 6, 23);

        check_graph.width_graph_traversal(1, 6);
    }
}