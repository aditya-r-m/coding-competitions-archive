lib::run!();

struct TestCase {
    v: Vec<i64>,
    w: Vec<i64>,
}

fn read() -> TestCase {
    lib::input!(_n);
    lib::input!(Vec<i64> as v);
    lib::input!(Vec<i64> as w);
    TestCase { v, w }
}

fn solve(TestCase { mut v, mut w }: TestCase) -> String {
    v.sort();
    w.sort();
    w.reverse();
    format!(
        "{}",
        v.into_iter()
            .zip(w.into_iter())
            .map(|(x, y)| x * y)
            .sum::<i64>()
    )
}
