use std::time::Instant;

use algorithms::geometric_algorithms::{line_segment_intersection, LineSegment};

mod algorithms;
mod data_structures;
mod exercises;

fn main() {
    let mut line_segments = Vec::new();

    line_segments.push(LineSegment::new((2, -3), (2, 3)));
    line_segments.push(LineSegment::new((3, -2), (15, -2)));
    line_segments.push(LineSegment::new((13, 0), (13, -3)));
    line_segments.push(LineSegment::new((0, 0), (4, 0)));
    line_segments.push(LineSegment::new((1, 1), (7, 1)));

    let intersections = line_segment_intersection(&line_segments);

    println!("{:#?}", intersections);
}
