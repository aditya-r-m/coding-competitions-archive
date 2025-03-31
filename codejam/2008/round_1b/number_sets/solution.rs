lib::run!();

struct TestCase {
    a: usize,
    b: usize,
    p: usize,
}

fn read() -> TestCase {
    lib::input!([usize; 3] as [a, b, p]);
    TestCase { a, b, p }
}

fn solve(TestCase { a, b, p }: TestCase) -> String {
    let mut hs = std::collections::HashSet::new();
    let mut ds = lib::collections::DisjointSet::new(1 + b - a);
    for e in (p..2 + b - a).filter(|&n| lib::algorithms::is_probable_prime(n)) {
        for i in (e * ((a + e - 1) / e)..1 + b - e).step_by(e) {
            ds.union(i - a, i + e - a);
        }
    }
    for i in a..1 + b {
        hs.insert(ds.get_root(i - a));
    }
    format!("{}", hs.len())
}
