lib::run!();

fn read() -> (usize, usize) {
    lib::input!((usize, usize) as (c, n));
    (c, n)
}

fn generate_transformation(c: usize, n: usize) -> Vec<Vec<f64>> {
    let mut factorials = vec![1f64; 1 + c];
    for i in 1..1 + c {
        factorials[i] = (i as f64) * factorials[i - 1];
    }
    let mut combinations = vec![vec![0f64; 1 + c]; 1 + c];
    for i in 0..1 + c {
        for j in 0..1 + i {
            combinations[i][j] = factorials[i] / (factorials[j] * factorials[i - j]);
        }
    }
    let mut transformation: Vec<Vec<f64>> = vec![vec![0f64; 1 + c]; 1 + c];
    for i in 0..1 + c {
        for j in i..1 + std::cmp::min(c, n + i) {
            transformation[i][j] =
                (combinations[i][n - (j - i)] * combinations[c - i][j - i]) / combinations[c][n];
        }
    }
    transformation
}

fn solve((c, n): (usize, usize)) -> String {
    let mut cur_expected_value: Vec<f64> = vec![1f64; c];
    cur_expected_value.push(0f64);
    let mut nxt_expected_value = cur_expected_value.clone();
    let transformation = generate_transformation(c, n);
    loop {
        for i in 0..c {
            nxt_expected_value[i] = 1f64;
            for j in i..1 + std::cmp::min(c, n + i) {
                nxt_expected_value[i] += transformation[i][j] * cur_expected_value[j];
            }
        }
        if (nxt_expected_value[0] - cur_expected_value[0]).abs() < 1e-10 {
            break;
        }
        (cur_expected_value, nxt_expected_value) = (nxt_expected_value, cur_expected_value);
    }
    format!("{:.7}", nxt_expected_value[0])
}
