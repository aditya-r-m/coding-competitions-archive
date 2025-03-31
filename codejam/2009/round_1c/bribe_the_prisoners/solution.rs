lib::run!();

fn read() -> Vec<usize> {
    lib::input!([usize; 2] as [p, _]);
    lib::input!(Vec<usize> as qs);
    qs.insert(0, 0);
    qs.push(p + 1);
    qs
}

fn solve(qs: Vec<usize>) -> String {
    let mut cache = vec![vec![0usize; qs.len()]; qs.len()];
    for l in 2..qs.len() {
        for i in 0..qs.len() - l {
            let k = i + l;
            cache[i][k] = usize::MAX;
            for j in i + 1..k {
                cache[i][k] =
                    std::cmp::min(cache[i][k], qs[k] - qs[i] - 2 + cache[i][j] + cache[j][k]);
            }
        }
    }
    format!("{}", cache[0].last().unwrap())
}
