use std::collections::HashSet;

fn is_happy(
    n: usize,
    base: usize,
    happy_set: &mut HashSet<usize>,
    visited_set: &mut HashSet<usize>,
) -> bool {
    if n == 1 {
        return true;
    }
    if !visited_set.contains(&n) {
        visited_set.insert(n);
        let mut m = 0usize;
        let mut i = n;
        while i > 0 {
            m += (i % base) * (i % base);
            i /= base;
        }
        if is_happy(m, base, happy_set, visited_set) {
            happy_set.insert(n);
        }
    }
    return happy_set.contains(&n);
}

fn main() {
    lib::input!(usize as test_case_count);
    let mut happy_sets = vec![HashSet::<usize>::new(); 11];
    let mut visited_sets = vec![HashSet::<usize>::new(); 11];
    for test_case_index in 1..1 + test_case_count {
        lib::input!(Vec<usize> as bases);
        let mut n = 1;
        loop {
            n += 1;
            let mut all_happy = true;
            for &base in bases.iter() {
                if !is_happy(n, base, &mut happy_sets[base], &mut visited_sets[base]) {
                    all_happy = false;
                }
            }
            if all_happy {
                println!("Case #{test_case_index}: {n}");
                break;
            }
        }
    }
}
