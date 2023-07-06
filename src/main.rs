use std::time::Instant;

use crate::{
    algorithms::{knuth_shuffle::shuffle, merge_sort as msort, quick_sort as qsort},
    data_structures::queue::Queue,
    exercises::collinear_points::fast_collinear_points,
};

mod algorithms;
mod data_structures;
mod exercises;

fn main() {
    // let points = vec![
    //     (1.0, 2.0),
    //     (1.0, 3.0),
    //     (2.0, 4.0),
    //     (-1.0, 0.0),
    //     (3.0, 6.0),
    //     (4.0, 8.0),
    // ];

    // let lines = fast_collinear_points(points);

    // println!("{:#?}", lines);

    let mut v: Vec<_> = (0..=100000000).collect();
    let sorting = Instant::now();
    let kth_element = qsort::selection(&mut v, 56000);
    println!("{:#?}", kth_element);
    println!("{:#?}", v.last());
    println!("{:#?}", sorting.elapsed());
}
