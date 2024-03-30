lib::run!();

const M: usize = 30031;

fn mod_int(i: usize) -> lib::collections::ModInt {
    lib::collections::ModInt::new(i, M)
}

struct TestCase {
    n: usize,
    k: usize,
    p: usize,
}

fn read() -> TestCase {
    lib::input!([usize; 3] as [n, k, p]);
    TestCase { n, k, p }
}

fn solve(TestCase { n, k, p }: TestCase) -> String {
    let mut transformation =
        lib::collections::SquareMatrix::new(vec![vec![mod_int(0); 1 << (p - 1)]; 1 << (p - 1)]);
    for s in 0..1 << (p - 1) {
        for j in 0..1 + (p - 1) * (s & 1) {
            transformation.rows[s][(s | (1 << j)) >> 1] = mod_int(1 - ((s >> j) & 1));
        }
    }
    format!(
        "{}",
        lib::algorithms::exp(transformation, n - k).rows[(1 << (k - 1)) - 1][(1 << (k - 1)) - 1].i
    )
}
