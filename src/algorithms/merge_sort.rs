use crate::algorithms::insertion_sort as isort;
static OFFSET: usize = 7;

pub fn sort<T: PartialOrd>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= OFFSET {
        isort::sort(&mut v);
        return v;
    }

    let mid = v.len() / 2;

    let sorted_right = sort(v.split_off(mid));
    let sorted_left = sort(v);

    merge(sorted_left, sorted_right)
}

fn merge<T: PartialOrd>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
    let merged_len = v1.len() + v2.len();

    let mut merged_v = Vec::with_capacity(merged_len);

    let mut v1 = v1.into_iter();
    let mut v2 = v2.into_iter();

    let mut e1 = v1.next();
    let mut e2 = v2.next();

    while e1.is_some() || e2.is_some() {
        if e1.is_none() {
            merged_v.push(e2.take().unwrap());
            e2 = v2.next();
        } else if e2.is_none() {
            merged_v.push(e1.take().unwrap());
            e1 = v1.next();
        } else {
            let val1 = e1.take().unwrap();
            let val2 = e2.take().unwrap();

            if val1 <= val2 {
                merged_v.push(val1);
                e1 = v1.next();
                e2 = Some(val2);
            } else {
                merged_v.push(val2);
                e2 = v2.next();
                e1 = Some(val1);
            }
        }
    }

    merged_v
}
