lib::run!();

struct TestCase {
    n: usize,
    m: usize,
    a: usize,
}

fn read() -> TestCase {
    lib::input!([usize; 3] as [n, m, a]);
    TestCase { n, m, a }
}

fn solve(TestCase { n, m, a }: TestCase) -> String {
    if n * m < a {
        "IMPOSSIBLE".to_string()
    } else {
        let [x1, y1, x2, y2]: [usize; 4];
        if a % n == 0 {
            y2 = a / n;
            x1 = n;
            x2 = 0;
            y1 = 0;
        } else {
            y2 = 1 + a / n;
            x1 = 1 + a / y2;
            x2 = 1;
            y1 = (x1 * y2) - a;
        }
        assert!(x1 * y2 - x2 * y1 == a);
        assert!(x1 <= n);
        assert!(x2 <= n);
        assert!(y1 <= m);
        assert!(y2 <= m);
        format!("0 0 {x1} {y1} {x2} {y2}")
    }
}
