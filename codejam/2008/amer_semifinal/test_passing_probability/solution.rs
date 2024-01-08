lib::run!();

struct TestCase {
    m: usize,
    ps: Vec<Vec<f64>>,
}

fn read() -> TestCase {
    lib::input!([usize; 2] as [m, q]);
    let mut ps: Vec<Vec<f64>> = Vec::new();
    for _ in 0..q {
        lib::input!(Vec<f64> as p);
        ps.push(p);
    }
    TestCase { m, ps }
}

fn solve(TestCase { m, ps }: TestCase) -> String {
    format!("{:.9}", recur(m, ps).into_iter().sum::<f64>())
}

fn recur(m: usize, mut ps: Vec<Vec<f64>>) -> Vec<f64> {
    if let Some(p) = ps.pop() {
        let mut result: Vec<f64> = Vec::new();
        for y in recur(m, ps) {
            for &x in p.iter() {
                result.push(x * y);
            }
        }
        result.sort_by(|b, a| a.partial_cmp(b).unwrap());
        result.into_iter().take(m).collect::<Vec<f64>>()
    } else {
        vec![1f64]
    }
}
