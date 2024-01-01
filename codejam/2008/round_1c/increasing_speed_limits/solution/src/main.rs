use std::collections::HashMap;
use std::convert::TryInto;

const M: usize = 1_000_000_007usize;

struct BiTree {
    values: Vec<usize>,
}

impl BiTree {
    fn new(n: usize) -> BiTree {
        BiTree { values: vec![0; n] }
    }

    fn add(&mut self, mut i: usize, v: usize) {
        if i == 0 {
            self.values[0] += v;
            self.values[0] %= M;
            return;
        }
        while i < self.values.len() {
            self.values[i] += v;
            self.values[0] %= M;
            i += i & (!i + 1);
        }
    }

    fn get(&self, mut i: usize) -> usize {
        let mut result = self.values[0];

        while i > 0 {
            result += self.values[i];
            result %= M;
            i -= i & (!i + 1);
        }

        result
    }
}

fn compress(speed_limits: Vec<usize>) -> Vec<usize> {
    let mut compressed_speed_limits = vec![speed_limits.len(); speed_limits.len()];

    let mut sorted_speed_limits = speed_limits.clone();
    sorted_speed_limits.sort();

    let compression_map = std::iter::zip(sorted_speed_limits, 0..speed_limits.len())
        .collect::<HashMap<usize, usize>>();
    for (i, v) in speed_limits.iter().enumerate() {
        compressed_speed_limits[i] = compression_map[v];
    }

    compressed_speed_limits
}

fn solve(mut speed_limits: Vec<usize>) -> usize {
    speed_limits = compress(speed_limits);

    let mut cache = BiTree::new(speed_limits.len());
    for v in speed_limits.into_iter() {
        cache.add(v, 1);
        if v > 0 {
            cache.add(v, cache.get(v - 1));
        }
    }
    cache.get(cache.values.len() - 1)
}

fn main() {
    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer).unwrap();
    let test_case_count = buffer.trim().parse::<usize>().unwrap();
    buffer.clear();

    for test_case_index in 1..1 + test_case_count {
        std::io::stdin().read_line(&mut buffer).unwrap();
        let [n, m, x, y, z]: [usize; 5] = buffer
            .split(" ")
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();
        buffer.clear();

        let mut a: Vec<usize> = Vec::new();
        a.reserve(m);
        for _ in 0..m {
            std::io::stdin().read_line(&mut buffer).unwrap();
            a.push(buffer.trim().parse::<usize>().unwrap());
            buffer.clear();
        }

        let mut speed_limits: Vec<usize> = vec![0; n];
        for i in 0..n {
            speed_limits[i] = a[i % m];
            a[i % m] = (x * a[i % m] + y * (i + 1)) % z;
        }

        println!("Case #{test_case_index}: {}", solve(speed_limits));
    }
}
