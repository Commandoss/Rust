extern crate Graph;

use Graph::direct::libGraph;
use std::collections::HashMap;

fn create_new_vertex() -> HashMap<i32, &'static str> {
    let mut HS = HashMap::new();
    HS.insert(0, "Zero");
    HS.insert(1, "First");
    HS.insert(2, "Second");
    HS.insert(3, "Third");
    HS
}

fn main() {
    let mut graph = libGraph::Graph::create_graph( create_new_vertex());
    graph.add_node(32, "thirty two");
    graph.print_vertex();
    graph.delete_node(2);
    graph.print_vertex();
    graph.delete_graph();
    graph.print_vertex();
}
