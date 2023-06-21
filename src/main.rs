// use crate::data_structures::queue::Queue;

use exercises::percolation::execute_percolation;

mod algorithms;
mod data_structures;
mod exercises;

fn main() {
    // let mut my_list = Queue::new();
    //
    // my_list.enqueue(4);
    // my_list.enqueue(3);
    // my_list.enqueue(1);
    //
    // println!("{:#?}", my_list);
    //
    // println!("{:#?}", my_list.dequeue());
    //
    // println!("{:#?}", my_list)
    execute_percolation(5, 1)
}
