const M: usize = 1_000_000_007usize;

lib::run!();

fn read() -> Vec<usize> {
    lib::input!([usize;5] as [n, m, x, y, z]);
    let mut a: Vec<usize> = Vec::new();
    for _ in 0..m {
        lib::input!(usize as v);
        a.push(v);
    }
    let mut speed_limits: Vec<usize> = vec![0; n];
    for i in 0..n {
        speed_limits[i] = a[i % m];
        a[i % m] = (x * a[i % m] + y * (i + 1)) % z;
    }
    speed_limits
}

fn solve(mut speed_limits: Vec<usize>) -> String {
    speed_limits = compress(speed_limits);
    let mod_int = |i| lib::collections::ModInt { i, m: M };
    let n = speed_limits.len();
    let mut cache = lib::collections::BinaryIndexedTree::new(mod_int(0), n);
    for v in speed_limits.into_iter() {
        cache.add(v, mod_int(1));
        if v > 0 {
            cache.add(v, cache.get(v - 1));
        }
    }
    format!("{}", cache.get(n - 1).i)
}

fn compress(speed_limits: Vec<usize>) -> Vec<usize> {
    let mut compressed_speed_limits = vec![speed_limits.len(); speed_limits.len()];

    let mut sorted_speed_limits = speed_limits.clone();
    sorted_speed_limits.sort();

    let compression_map = std::iter::zip(sorted_speed_limits, 0..speed_limits.len())
        .collect::<std::collections::HashMap<usize, usize>>();
    for (i, v) in speed_limits.iter().enumerate() {
        compressed_speed_limits[i] = compression_map[v];
    }

    compressed_speed_limits
}
