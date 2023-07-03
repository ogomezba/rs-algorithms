use std::cmp::min;

use crate::algorithms::insertion_sort as isort;

use super::insertion_sort::sort_section;
static CUTOFF: usize = 7;

pub fn sort<T: Ord + Copy>(v: &mut Vec<T>) {
    basic_sort(v, &mut v.clone(), 0, v.len() - 1);
}

pub fn sort_without_copy<T: Ord>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= CUTOFF {
        isort::sort(&mut v);
        return v;
    }

    let mid = v.len() / 2;

    let sorted_right = sort_without_copy(v.split_off(mid));
    let sorted_left = sort_without_copy(v);

    merge_without_copy(sorted_left, sorted_right)
}

pub fn basic_sort<T: Ord + Copy>(v: &mut Vec<T>, aux: &mut Vec<T>, low: usize, high: usize) {
    if high - low <= CUTOFF {
        sort_section(v, low, high);
        return;
    }

    let mid = low + (high - low) / 2;

    basic_sort(v, aux, low, mid);
    basic_sort(v, aux, mid + 1, high);

    in_place_merge(v, aux, low, high, mid);
}

pub fn bottom_up_sort<T: Ord + Copy>(v: &mut Vec<T>) {
    let mut sz = 1;
    let mut aux = v.clone();
    let N = v.len();

    while sz < N {
        let mut j = 0;

        while j < N - sz {
            in_place_merge(v, &mut aux, j, min(j + 2 * sz - 1, N - 1), j + sz - 1);
            j += 2 * sz;
        }

        sz *= 2;
    }
}

pub fn in_place_merge<T: Ord + Copy>(
    v: &mut Vec<T>,
    aux: &mut Vec<T>,
    low: usize,
    high: usize,
    mid: usize,
) {
    let mut l = low;

    while l <= high {
        aux[l] = v[l];
        l += 1;
    }

    let mut k = low;
    let mut i = low;
    let mut j = mid + 1;

    while k <= high {
        if i > mid {
            v[k] = aux[j];
            j += 1;
        } else if j > high {
            v[k] = aux[i];
            i += 1;
        } else {
            if aux[i] <= aux[j] {
                v[k] = aux[i];
                i += 1;
            } else {
                v[k] = aux[j];
                j += 1;
            }
        }
        k += 1;
    }
}

fn merge_without_copy<T: PartialOrd>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
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
