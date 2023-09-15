use std::time::Instant;

use rand::{seq::SliceRandom, thread_rng};

use crate::data_structures::binary_search_tree::BinarySearchTree;

mod algorithms;
mod data_structures;
mod exercises;

fn main() {
    let mut rng = thread_rng();
    let mut numbers: Vec<usize> = (1..=1000000).collect();
    numbers.shuffle(&mut rng);

    let mut bst = BinarySearchTree::new();

    for a in numbers {
        bst.put(a, a);
    }

    let mins = Instant::now();

    println!("{:#?}", bst.get(5));
    println!("{:#?}", bst.get(1000));
    println!("{:#?}", bst.len());
    bst.delete(5);
    println!("{:#?}", bst.len());
    bst.delete(1000);
    println!("{:#?}", bst.get(5));
    println!("{:#?}", bst.get(1000));
    println!("{:#?}", bst.get(1));
    println!("{:#?}", bst.get(50000));
    println!("{:#?}", bst.len());
    println!("{:#?}", mins.elapsed());
}
