pub fn sort<T: PartialOrd>(v: &mut Vec<T>) {
    let mut i = 0;

    while i < v.len() {
        let mut j = i;
        let mut min_idx = i;

        while j < v.len() {
            if v[j] < v[min_idx] {
                min_idx = j;
            }

            j += 1;
        }

        v.swap(i, min_idx);
        i += 1;
    }
}
