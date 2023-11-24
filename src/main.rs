use data_structures::graph::Graph;

use crate::algorithms::graph_algorithms::BreadthFirstPath;

mod algorithms;
mod data_structures;
mod exercises;

fn main() {
    let mut graph = Graph::new(12);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(2, 3);
    graph.add_edge(5, 2);
    graph.add_edge(8, 9);
    graph.add_edge(1, 5);

    let paths = BreadthFirstPath::create(&graph, 0);
    println!("{:#?}", paths.path_to(1));
    println!("{:#?}", paths.path_to(2));
    println!("{:#?}", paths.path_to(3));
    println!("{:#?}", paths.path_to(4));
    println!("{:#?}", paths.path_to(5));
    println!("{:#?}", paths.path_to(6));
    println!("{:#?}", paths.path_to(7));
    println!("{:#?}", paths.path_to(8));
    println!("{:#?}", paths.path_to(9));
    println!("{:#?}", paths.path_to(10));
}
