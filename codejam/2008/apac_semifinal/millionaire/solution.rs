lib::run!();

struct TestCase {
    m: usize,
    p: f64,
    x: f64,
}

fn read() -> TestCase {
    lib::input!((usize, f64, f64) as (m, p, x));
    TestCase { m, p, x }
}

fn solve(TestCase { m, p, x }: TestCase) -> String {
    let mut p_ranges: Vec<f64> = vec![0f64, 1f64];
    for _ in 0..m {
        let mut np_ranges: Vec<f64> = vec![0f64; 2 * p_ranges.len() - 1];
        for j in 0..np_ranges.len() {
            for k in (j & 1..j + 1).step_by(2) {
                np_ranges[j] = f64::max(
                    np_ranges[j],
                    p * p_ranges.get((j + k) >> 1).unwrap_or(&1f64)
                        + (1f64 - p) * p_ranges.get((j - k) >> 1).unwrap(),
                )
            }
        }
        p_ranges = np_ranges;
    }
    let l = (p_ranges.len() - 1) as f64;
    format!(
        "{:.9}",
        p_ranges
            .into_iter()
            .enumerate()
            .take_while(|&(r, _)| 1e6f64 * (r as f64) / l <= x)
            .last()
            .unwrap()
            .1
    )
}
