use rand::Rng;

pub fn shuffle<T>(v: &mut [T]) {
    let mut i = 1;
    let mut rnd = rand::thread_rng();

    while i < v.len() {
        let n = rnd.gen_range(0..=i);
        v.swap(i, n);
        i += 1;
    }
}
