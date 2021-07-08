extern crate Graph;

use Graph::direct::libGraph;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use Graph::test::cfg;
use std::cmp::Ordering::Greater;

fn create_new_vertex() -> HashMap<i32, char> {
    let mut hs = HashMap::new();
    hs.insert(0, 'g');
    hs.insert(1, 'F');
    hs.insert(2, 'S');
    hs.insert(3, 'T');
    hs.insert(4, 'F');
    hs.insert(5, 'F');
    hs.insert(6, 'S');
    hs.insert(7, 'S');
    hs
}

fn main()  {

    let mut graph = libGraph::Graph::create_graph(create_new_vertex());
    graph.add_node(32, '3');
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
    graph.delete_graph();
    graph.print_vertexs_direction();

    graph.read_from_file(path);
    graph.print_vertexs_direction();

    graph.delete_graph();

}
