extern crate Graph;

use Graph::direct::libGraph;
use std::collections::HashMap;

fn create_new_vertex() -> HashMap<i32, &'static str> {
    let mut hs = HashMap::new();
    hs.insert(0, "Zero");
    hs.insert(1, "First");
    hs.insert(2, "Second");
    hs.insert(3, "Third");
    hs.insert(4, "Four");
    hs.insert(5, "Five");
    hs.insert(6, "Six");
    hs.insert(7, "Seven");
    hs
}

fn main() {
    let mut graph = libGraph::Graph::create_graph( create_new_vertex());
    graph.add_node(32, "Thirty two");
    graph.delete_node(7);

    graph.add_oriented_rib(0, 1, 2);
    graph.add_oriented_rib(0, 2, 13);
    graph.add_oriented_rib(0, 3, 22);
    graph.add_oriented_rib(1, 2, 12);
    graph.add_oriented_rib(3, 2, 52);
    graph.add_oriented_rib(2, 4, 42);
    graph.add_oriented_rib(2, 5, 8);
    graph.add_oriented_rib(2, 6, 1);
    graph.add_oriented_rib(6, 32, 77);
    graph.print_vertexs_direction();

    println!("{}", graph.width_graph_traversal(0, 32));

    graph.delete_graph();
    graph.print_vertex();
}
