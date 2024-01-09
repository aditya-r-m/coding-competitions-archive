use std::collections::HashSet;

lib::run!();

struct TestCase {
    k: usize,
    s: Vec<char>,
}

fn read() -> TestCase {
    lib::input!(usize as k);
    lib::input!(Vec<char> as s);
    TestCase { k, s }
}

fn solve(TestCase { k, s }: TestCase) -> usize {
    let mut inner_cost: Vec<Vec<usize>> = vec![vec![0; k]; k];
    let mut outer_cost: Vec<Vec<usize>> = vec![vec![0; k]; k];
    for b in (0..s.len()).step_by(k) {
        for i in 0..k {
            for j in 0..k {
                inner_cost[i][j] += (s[b + i] != s[b + j]) as usize;
                if b < s.len() - k {
                    outer_cost[i][j] += (s[b + i] != s[b + k + j]) as usize;
                }
            }
        }
    }
    let mut cache: Vec<Vec<Vec<usize>>> = vec![vec![vec![usize::max_value(); k]; k]; 1 << k];
    let mut cur_layer: HashSet<usize> = HashSet::new();
    let mut nxt_layer: HashSet<usize> = HashSet::new();
    for i in 0..k {
        for j in 0..k {
            if i == j {
                continue;
            }
            let cur_set = (1 << i) | (1 << j);
            cur_layer.insert(cur_set);
            cache[cur_set][i][j] = inner_cost[i][j];
        }
    }
    while !cur_layer.is_empty() {
        for &cur_set in cur_layer.iter() {
            for e in 0..k {
                let nxt_set = cur_set | (1 << e);
                if nxt_set == cur_set {
                    continue;
                }
                nxt_layer.insert(nxt_set);
                for i in 0..k {
                    if cur_set & (1 << i) == 0 {
                        continue;
                    }
                    for j in 0..k {
                        if i == j || cur_set & (1 << j) == 0 {
                            continue;
                        }
                        cache[nxt_set][i][e] = std::cmp::min(
                            cache[nxt_set][i][e],
                            cache[cur_set][i][j] + inner_cost[j][e],
                        );
                    }
                }
            }
        }
        cur_layer.clear();
        [cur_layer, nxt_layer] = [nxt_layer, cur_layer];
    }
    let mut sol = usize::max_value();
    let l = (1 << k) - 1;
    for i in 0..k {
        for j in 0..k {
            if i == j {
                continue;
            }
            sol = std::cmp::min(sol, 1 + cache[l][i][j] + outer_cost[j][i]);
        }
    }
    sol
}
