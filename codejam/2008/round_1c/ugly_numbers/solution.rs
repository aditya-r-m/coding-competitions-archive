const M: usize = 2 * 3 * 5 * 7;

lib::run!();

fn read() -> Vec<char> {
    lib::input!(Vec<char> as cs);
    cs
}

fn solve(cs: Vec<char>) -> String {
    let mut cache: Vec<Vec<usize>> = vec![vec![0usize; M]; 1 + cs.len()];
    cache[0][0] = 1;
    for i in 0..cs.len() {
        let mut c = 0usize;
        for j in 1 + i..1 + cs.len() {
            c *= 10;
            c += cs[j - 1].to_digit(10).unwrap() as usize;
            c %= M;
            for x in 0..M {
                cache[j][(x + c) % M] += cache[i][x];
                if i > 0 {
                    cache[j][(M + x - c) % M] += cache[i][x];
                }
            }
        }
    }
    let mut sol = 0usize;
    for i in 0..M {
        if i % 2 == 0 || i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
            sol += cache[cs.len()][i];
        }
    }
    format!("{sol}")
}
