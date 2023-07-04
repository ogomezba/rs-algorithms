use std::time::Instant;

use algorithms::knuth_shuffle::shuffle;
use exercises::collinear_points::brute_collinear_points;

use crate::{
    algorithms::merge_sort::bottom_up_sort,
    exercises::{collinear_points::fast_collinear_points, percolation::execute_percolation},
};

mod algorithms;
mod data_structures;
mod exercises;

fn main() {
    let points = vec![
        (1.0, 2.0),
        (1.0, 3.0),
        (2.0, 4.0),
        (-1.0, 0.0),
        (3.0, 6.0),
        (4.0, 8.0),
    ];

    let lines = fast_collinear_points(points);

    println!("{:#?}", lines);

    // let mut v = (0..=100000000).collect();
    // let shuffling = Instant::now();
    // shuffle(&mut v);
    // println!("{:#?}", v.last());
    // println!("{:#?}", shuffling.elapsed());
    //
    // let sorting = Instant::now();
    // bottom_up_sort(&mut v);
    // println!("{:#?}", v.last());
    // println!("{:#?}", sorting.elapsed());
    //
    // execute_percolation(5, 1)
}
