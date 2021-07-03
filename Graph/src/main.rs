extern crate Graph;

use Graph::direct::libGraph;
use std::collections::HashMap;

fn create_new_vertex() -> HashMap<i32, &'static str> {
    let mut hs = HashMap::new();
    hs.insert(0, "Zero");
    hs.insert(1, "First");
    hs.insert(2, "Second");
    hs.insert(3, "Third");
    hs
}

fn main() {
    let mut graph = libGraph::Graph::create_graph( create_new_vertex());
    graph.add_node(32, "Thirty two");
    graph.print_vertex();

    graph.delete_node(2);
    graph.print_vertex();

    graph.add_oriented_rib(1, 32, 22);
    graph.add_oriented_rib(1, 3, 15);
    graph.add_oriented_rib(1, 1, 35);
    graph.print_vertexs_direction();

    graph.change_direcrion_rib(1, 32, 0);
    graph.print_vertexs_direction();

    graph.set_rib_weight(32, 1, 55);
    graph.print_vertexs_direction();

    graph.delete_graph();
    graph.print_vertex();
}
