use std::cmp::Ordering;

pub fn brute_collinear_points(points: Vec<Point>) -> Vec<LineSegment> {
    let mut segments = Vec::new();

    for (i, &p1) in points.iter().enumerate() {
        for (j, &p2) in points.iter().enumerate().skip(i + 1) {
            let slope1 = slope(p1, p2);

            for (k, &p3) in points.iter().enumerate().skip(j + 1) {
                let slope2 = slope(p1, p3);

                if slope1 != slope2 {
                    continue;
                }

                for &p4 in points.iter().skip(k + 1) {
                    let slope3 = slope(p1, p4);

                    if slope1 == slope3 {
                        segments.push(LineSegment::new(p1, p4));
                        //We break because as mentioned in the exercise, for the
                        //brute-force algorythm, more than 4 collinear points
                        //won't be sent
                        break;
                    }
                }
            }
        }
    }

    return segments;
}

pub fn fast_collinear_points(points: Vec<Point>) -> Vec<LineSegment> {
    let mut segments = Vec::new();

    let mut points = points.clone();
    points.sort_unstable_by(compare_height);

    for (i, &p) in points.iter().enumerate() {
        let mut rest: Vec<(Point, f32)> = points
            .iter()
            .skip(i + 1)
            .map(|&q| (q, slope(p, q)))
            .collect();

        rest.sort_by(|(_, slope1), (_, slope2)| slope1.total_cmp(slope2));

        for (j, &(_, slope)) in rest.iter().enumerate() {
            let mut collinear_count = 1;

            for &(_, next_slope) in rest.iter().skip(j + 1) {
                if next_slope == slope {
                    collinear_count += 1;
                } else {
                    break;
                }
            }

            if collinear_count >= 3 {
                segments.push(LineSegment::new(p, rest[j + collinear_count - 1].0));
            }
        }
    }

    return segments;
}

pub type Point = (f32, f32);

#[derive(Debug)]
pub struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}

fn compare_height((x1, y1): &Point, (x2, y2): &Point) -> Ordering {
    match (y1.total_cmp(y2), x1.total_cmp(x2)) {
        (Ordering::Greater, _) => Ordering::Greater,
        (Ordering::Less, _) => Ordering::Less,
        (Ordering::Equal, Ordering::Greater) => Ordering::Greater,
        (Ordering::Equal, Ordering::Less) => Ordering::Less,
        _ => Ordering::Equal,
    }
}

fn slope((x1, y1): Point, (x2, y2): Point) -> f32 {
    if x1 == x2 {
        if y1 == y2 {
            return f32::NEG_INFINITY;
        } else {
            return f32::INFINITY;
        }
    }

    ((y2 - y1) / (x2 - x1)).abs()
}
