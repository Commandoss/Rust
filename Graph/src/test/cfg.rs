
#[cfg(test)]
mod tests {
    use crate::direct::libGraph::{Graph, Node};

    #[test]
    fn create_vetrex() {
        Node::new_node(23, "Check create vertex!");
        Node::new_node("23", "Check create vertex!");
        Node::new_node(23.0, "Check create vertex!");
        Node::new_node(false, "Check create vertex!");
    }

    #[test]
    fn create_graph_32_str() {
        let mut check_graph: Graph<u32, &str> = Graph::new();
        check_graph.add_node(1, "one");
        check_graph.add_node(2, "two");
        check_graph.add_node(3, "three");
        check_graph.add_node(4, "four");
        check_graph.add_node(5, "five");
        check_graph.add_node(6, "six");
        check_graph.delete_graph();
    }

    #[test]
    fn create_graph_str_str() {
        let mut check_graph: Graph<&str, &str> = Graph::new();
        check_graph.add_node("1.22", "one");
        check_graph.add_node("2.32", "two");
        check_graph.add_node("13.0", "three");
        check_graph.add_node("41.3", "four");
        check_graph.add_node("5555.006", "five");
        check_graph.add_node("61231.555", "six");
        check_graph.delete_graph();
    }

    // #[test]
    // fn create_graph_f32_str() {
    //     let mut check_graph: Graph<f64, &str> = Graph::new();
    //     // check_graph
    //     // check_graph.add_node(21.22, "one");
    //     // check_graph.add_node(2.32, "two");
    //     // check_graph.add_node(13.0, "three");
    //     // check_graph.add_node(41.3, "four");
    //     // check_graph.add_node(5555.006, "five");
    //     // check_graph.add_node(61231.555, "six");
    //     // check_graph.delete_graph();
    // }

    #[test]
    fn add_oriented_rib() {
        let mut check_graph: Graph<u32, &str> = Graph::new();
        check_graph.add_node(1, "one");
        check_graph.add_node(2, "two");

        check_graph.add_oriented_rib(1, 2, 23);
        check_graph.delete_graph();
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
        check_graph.delete_graph();
    }

    #[test]
    fn delete() {
        let mut check_graph: Graph<u32, &str> = Graph::new();
        check_graph.add_node(1, "one");
        check_graph.add_node(2, "two");
        check_graph.add_node(3, "three");
        check_graph.add_node(4, "four");
        check_graph.add_node(5, "five");
        check_graph.add_node(6, "six");

        check_graph.delete_graph();
    }

    #[test]
    fn delete_and_traversal() {
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

        check_graph.delete_oriented_rib(5, 6);
        check_graph.delete_graph();

        check_graph.width_graph_traversal(1, 6);
    }

    #[test]
    fn print() {
        let mut check_graph: Graph<u32, &str> = Graph::new();
        check_graph.add_node(1, "one");
        check_graph.add_node(2, "two");
        check_graph.add_node(3, "three");
        check_graph.add_node(4, "four");
        check_graph.add_node(5, "five");
        check_graph.add_node(6, "six");

        check_graph.print_vertex();
        check_graph.delete_graph();
    }

    #[test]
    fn write_file() {
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

        check_graph.write_to_file("test.txt");
        check_graph.delete_graph();
    }
}