use super::shell_sort::shell_sort_h;

pub fn sort<T: PartialOrd>(v: &mut Vec<T>) {
    shell_sort_h(v, 1);
}

pub fn sort_section<T: PartialOrd>(v: &mut Vec<T>, low: usize, high: usize) {
    let mut i = low + 1;

    while i <= high {
        let mut j = i;

        while j > low {
            if v[j] < v[j - 1] {
                v.swap(j, j - 1);
                j -= 1;
            } else {
                break;
            }
        }

        i += 1;
    }
}
