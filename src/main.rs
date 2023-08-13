use std::time::Instant;

use algorithms::knuth_shuffle::shuffle;
use data_structures::binary_heap::BinaryHeap;
use exercises::slider_puzzle::{self, Board};

use crate::{
    algorithms::heap_sort,
    data_structures::binary_heap::{max_priority_queue, min_priority_queue},
};

mod algorithms;
mod data_structures;
mod exercises;

fn main() {
    let tiles_1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]];
    let tiles_2 = vec![vec![2, 1, 3], vec![4, 5, 6], vec![7, 8, 0]];
    let tiles_3 = vec![vec![2, 0, 4], vec![6, 5, 3], vec![7, 8, 1]];
    let tiles_4 = vec![vec![0, 1, 3], vec![4, 2, 5], vec![7, 8, 6]];

    let board = Board::new(tiles_4);

    let solution = slider_puzzle::solve_puzzle(board);

    solution.print_solution();
}
