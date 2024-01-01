use std::collections::HashSet;
use std::convert::TryInto;

struct DisjointSet {
    links: Vec<usize>,
    ranks: Vec<usize>,
}

impl DisjointSet {
    fn new(sz: usize) -> DisjointSet {
        DisjointSet {
            links: (0..sz).collect::<Vec<usize>>(),
            ranks: vec![0; sz],
        }
    }

    fn get_root(&mut self, u: usize) -> usize {
        if self.links[u] != u {
            self.links[u] = self.get_root(self.links[u])
        }
        self.links[u]
    }

    fn union(&mut self, u_: usize, v_: usize) {
        let u = self.get_root(u_);
        let v = self.get_root(v_);
        if self.ranks[u] > self.ranks[v] {
            self.links[v] = u;
            return;
        }
        self.links[u] = v;
        if self.ranks[v] == self.ranks[u] {
            self.ranks[v] += 1;
        }
    }
}

fn get_primes(lb: usize, ub: usize) -> Vec<usize> {
    let mut sieve = vec![true; 1 + ub];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..1 + ub {
        if sieve[i] {
            for j in (2 * i..1 + ub).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    let mut primes = Vec::new();
    for p in lb..1 + ub {
        if sieve[p] {
            primes.push(p);
        }
    }

    primes
}

fn solve(a: usize, b: usize, p: usize) -> usize {
    let mut ds = DisjointSet::new(b - a + 1);
    let mut s = HashSet::new();

    for e in get_primes(p, 1 + b - a) {
        for i in (e * ((a + e - 1) / e)..1 + b - e).step_by(e) {
            ds.union(i - a, i + e - a);
        }
    }
    for i in a..1 + b {
        s.insert(ds.get_root(i - a));
    }

    s.len()
}

fn main() {
    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer).unwrap();
    let test_case_count = buffer.trim().parse::<usize>().unwrap();
    buffer.clear();

    for test_case_index in 1..1 + test_case_count {
        std::io::stdin().read_line(&mut buffer).unwrap();
        let [a, b, p]: [usize; 3] = buffer
            .split(" ")
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();
        buffer.clear();

        println!("Case #{test_case_index}: {}", solve(a, b, p));
    }
}
