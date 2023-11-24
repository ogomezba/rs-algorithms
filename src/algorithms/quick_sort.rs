use std::time::Instant;

use super::{insertion_sort as isort, knuth_shuffle::shuffle};

static CUTOFF: usize = 10;

pub fn sort<T: Ord>(v: &mut [T]) {
    do_sort(v);
}

pub fn selection<T: Ord>(v: &mut [T], k: usize) -> &T {
    if k >= v.len() {
        panic!("k can't be higher or equal to the total to the total amount of elements");
    }

    shuffle(v);
    do_selection(v, k, 0, v.len() - 1)
}

pub fn do_selection<T: Ord>(v: &mut [T], k: usize, lo: usize, hi: usize) -> &T {
    let partition = partition_full(v, lo, hi);

    match partition {
        n if n == k => return &v[n],
        n if n > k => return do_selection(v, k, lo, n - 1),
        n => return do_selection(v, k, n + 1, hi),
    }
}

fn do_sort<T: Ord>(v: &mut [T]) {
    let partition = partition_slice(v);

    if partition == 0 {
        return;
    }

    do_sort(&mut v[0..partition]);
    do_sort(&mut v[(partition + 1)..]);
}

fn partition_full<T: Ord>(v: &mut [T], lo: usize, hi: usize) -> usize {
    let mut i = lo + 1;
    let mut j = hi;

    loop {
        let pivot = &v[lo];

        while i < v.len() && &v[i] < pivot {
            i += 1;
        }

        while &v[j] > pivot {
            j -= 1;
        }

        if i >= j {
            break;
        }

        v.swap(i, j);
        i += 1;
        j -= 1;
    }

    v.swap(lo, j);

    return j;
}

fn partition_slice<T: Ord>(v: &mut [T]) -> usize {
    if v.is_empty() {
        return 0;
    }

    let mut idx = 0;
    let mut i = 0;
    let last = v.len() - 1;

    while i < last {
        if v[i] < v[last] {
            v.swap(i, idx);
            idx += 1;
        }

        i += 1;
    }

    v.swap(idx, last);
    return idx;
}
