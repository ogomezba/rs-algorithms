use std::time::Instant;

use data_structures::balanced_search_tree::BalancedSearchTree;

mod algorithms;
mod data_structures;
mod exercises;

fn main() {
    let mut bst = BalancedSearchTree::new();

    let put = Instant::now();

    for i in 0..=10000000 {
        bst.put(i, i)
    }

    println!("{:#?}", put.elapsed());

    let mut get = Instant::now();

    println!("{:#?}", bst.get(0));
    println!("{:#?}", get.elapsed());

    get = Instant::now();
    println!("{:#?}", bst.get(50));
    println!("{:#?}", get.elapsed());

    get = Instant::now();

    println!("{:#?}", bst.get(900000));
    println!("{:#?}", get.elapsed());
}
