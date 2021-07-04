extern crate Graph;

use Graph::direct::libGraph;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

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

fn main()  {
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

    println!("{}", graph.width_graph_traversal(32, 0));

    graph.delete_node(2);
    graph.print_vertexs_direction();

    let path: &str = "lines.txt";
    graph.write_to_file(path);
    graph.read_from_file(path);

    graph.delete_graph();
    graph.print_vertex();



}
