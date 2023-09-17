use std::mem::swap;

use crate::data_structures::{binary_heap::BinaryHeap, binary_search_tree::BinarySearchTree};

pub fn line_segment_intersection(segments: &[LineSegment]) -> Vec<Point> {
    let mut bh = BinaryHeap::new(less_prio_event);

    for segment in segments {
        if is_horizontal(segment) {
            bh.insert(Event::HorizontalStart {
                x: segment.start.0,
                y: segment.start.1,
            });
            bh.insert(Event::HorizontalEnd {
                x: segment.end.0,
                y: segment.end.1,
            });
        } else {
            let x = segment.start.0;
            let mut y1 = segment.start.1;
            let mut y2 = segment.end.1;

            if y1 > y2 {
                swap(&mut y1, &mut y2);
            }

            bh.insert(Event::Vertical { x, y1, y2 })
        }
    }

    let mut bst = BinarySearchTree::new();
    let mut intersections = Vec::with_capacity(segments.len());

    while !bh.is_empty() {
        let event = bh.delete_max();

        match event {
            Event::HorizontalStart { y, .. } => bst.put(y, y),
            Event::HorizontalEnd { y, .. } => bst.delete(y),
            Event::Vertical { x, y1, y2 } => {
                let search_result = bst.search_1d(&y1, &y2);

                for &y in search_result {
                    intersections.push((x, y));
                }
            }
        }
    }

    return intersections;
}

fn is_horizontal(segment: &LineSegment) -> bool {
    segment.start.1 == segment.end.1
}

fn less_prio_event(e1: &Event, e2: &Event) -> bool {
    match e1 {
        Event::HorizontalStart { x: x1, .. } => match e2 {
            Event::HorizontalStart { x: x2, .. } => x1 > x2,
            Event::HorizontalEnd { x: x2, .. } => x1 > x2,
            Event::Vertical { x: x2, .. } => x1 > x2,
        },
        Event::HorizontalEnd { x: x1, .. } => match e2 {
            Event::HorizontalStart { x: x2, .. } => x1 > x2,
            Event::HorizontalEnd { x: x2, .. } => x1 > x2,
            Event::Vertical { x: x2, .. } => x1 > x2,
        },
        Event::Vertical { x: x1, .. } => match e2 {
            Event::HorizontalStart { x: x2, .. } => x1 > x2,
            Event::HorizontalEnd { x: x2, .. } => x1 > x2,
            Event::Vertical { x: x2, .. } => x1 > x2,
        },
    }
}

enum Event {
    HorizontalStart { x: isize, y: isize },
    HorizontalEnd { x: isize, y: isize },
    Vertical { x: isize, y1: isize, y2: isize },
}

pub struct LineSegment {
    pub start: Point,
    pub end: Point,
}

impl LineSegment {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}

type Point = (isize, isize);
