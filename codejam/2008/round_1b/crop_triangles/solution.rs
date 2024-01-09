lib::run!();

fn read() -> Vec<(usize, usize)> {
    lib::input!([usize; 8] as [n, a, b, c, d, mut x, mut y, m]);
    let mut test_case: Vec<(usize, usize)> = Vec::new();
    for _ in 0..n {
        test_case.push((x, y));
        x = (a * x + b) % m;
        y = (c * y + d) % m;
    }
    test_case
}

fn solve(test_case: Vec<(usize, usize)>) -> String {
    let mut sketch = [[0i64; 3]; 3];
    for (x, y) in test_case.into_iter() {
        sketch[x % 3][y % 3] += 1;
    }
    let mut count = 0;
    for row in sketch.iter() {
        for &v in row.iter() {
            count += (v * (v - 1) * (v - 2)) / 6;
        }
    }
    for i in 0..3 {
        count += sketch[i][0] * sketch[i][1] * sketch[i][2];
        count += sketch[0][i] * sketch[1][i] * sketch[2][i];
        count += sketch[0][i] * sketch[1][(i + 1) % 3] * sketch[2][(i + 2) % 3];
        count += sketch[0][i] * sketch[1][(i + 2) % 3] * sketch[2][(i + 1) % 3];
    }
    format!("{count}")
}
