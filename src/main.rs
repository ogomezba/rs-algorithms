use std::time::Instant;

use data_structures::binary_search_tree::BinarySearchTree;

use crate::algorithms::{heap_sort::sort, knuth_shuffle::shuffle};

mod algorithms;
mod data_structures;
mod exercises;

fn main() {
    /* let mut v: Vec<u32> = (0..=10000000).collect();
    let shuffling = Instant::now();
    shuffle(&mut v);
    println!("{:#?}", v.last());
    println!("{:#?}", shuffling.elapsed());

    let sorting = Instant::now();
    let sorted = sort(v);
    println!("{:#?}", sorted.last());
    println!("{:#?}", sorting.elapsed()); */

    let mut map = BinarySearchTree::new();

    map.put("C", 2);
    map.put("F", 5);
    map.put("A", 0);
    map.put("E", 4);
    map.put("B", 1);
    map.put("D", 3);
    map.put("G", 6);

    println!("{:#?}", map);
    println!("{:#?}", map.get("F"));
    println!("{:#?}", map.get("G"));
    println!("{:#?}", map.get("B"));
    println!("{:#?}", map.get("C"));
    println!("{:#?}", map.get("A"));
}
