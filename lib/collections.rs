pub struct DisjointSet {
    links: Vec<usize>,
    ranks: Vec<usize>,
}

impl DisjointSet {
    pub fn new(sz: usize) -> DisjointSet {
        DisjointSet {
            links: (0..sz).collect::<Vec<usize>>(),
            ranks: vec![0; sz],
        }
    }

    pub fn get_root(&mut self, u: usize) -> usize {
        if self.links[u] != u {
            self.links[u] = self.get_root(self.links[u])
        }
        self.links[u]
    }

    pub fn union(&mut self, u_: usize, v_: usize) {
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

pub struct BinaryIndexedTree {
    values: Vec<usize>,
    m: usize,
}

impl BinaryIndexedTree {
    pub fn new(n: usize, m: usize) -> BinaryIndexedTree {
        BinaryIndexedTree {
            values: vec![0; n],
            m,
        }
    }

    pub fn add(&mut self, mut i: usize, v: usize) {
        if i == 0 {
            self.values[0] += v;
            self.values[0] %= self.m;
            return;
        }
        while i < self.values.len() {
            self.values[i] += v;
            self.values[0] %= self.m;
            i += i & (!i + 1);
        }
    }

    pub fn get(&self, mut i: usize) -> usize {
        let mut result = self.values[0];

        while i > 0 {
            result += self.values[i];
            result %= self.m;
            i -= i & (!i + 1);
        }
        result
    }
}
