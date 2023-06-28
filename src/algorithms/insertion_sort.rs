use super::shell_sort::shell_sort_h;

pub fn sort<T: PartialOrd>(v: &mut Vec<T>) {
    shell_sort_h(v, 1);
}
