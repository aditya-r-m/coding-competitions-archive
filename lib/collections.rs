use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct ModInt {
    pub i: usize,
    pub m: usize,
}

impl ModInt {
    pub fn new(i: usize, m: usize) -> ModInt {
        ModInt { i: i % m, m }
    }
}

impl Add for ModInt {
    type Output = ModInt;
    fn add(self, other: ModInt) -> ModInt {
        assert!(self.m == other.m);
        ModInt {
            i: (self.i + other.i) % self.m,
            m: self.m,
        }
    }
}

impl Mul for ModInt {
    type Output = ModInt;
    fn mul(self, other: ModInt) -> ModInt {
        assert!(self.m == other.m);
        ModInt {
            i: (self.i * other.i) % self.m,
            m: self.m,
        }
    }
}

pub trait AddIdentity {
    fn add_identity(&self) -> Self;
}

impl AddIdentity for ModInt {
    fn add_identity(&self) -> Self {
        ModInt { i: 0, m: self.m }
    }
}

pub trait MulIdentity {
    fn mul_identity(&self) -> Self;
}

impl MulIdentity for ModInt {
    fn mul_identity(&self) -> Self {
        ModInt { i: 1, m: self.m }
    }
}

#[derive(Clone)]
pub struct SquareMatrix<T> {
    pub rows: Vec<Vec<T>>,
    n: usize,
}

impl<T> SquareMatrix<T> {
    pub fn new(rows: Vec<Vec<T>>) -> SquareMatrix<T> {
        let n = rows.len();
        for row in rows.iter() {
            assert!(n == row.len());
        }
        SquareMatrix { rows, n }
    }
}

impl<T> Mul<SquareMatrix<T>> for SquareMatrix<T>
where
    T: Add<Output = T> + Mul<Output = T> + AddIdentity + Copy,
{
    type Output = SquareMatrix<T>;
    fn mul(self, other: SquareMatrix<T>) -> SquareMatrix<T> {
        assert!(self.n == other.n);
        let mut rows = vec![vec![self.rows[0][0].add_identity(); self.n]; self.n];
        for i in 0..self.n {
            for j in 0..self.n {
                for k in 0..self.n {
                    rows[i][j] = rows[i][j] + self.rows[i][k] * other.rows[k][j];
                }
            }
        }
        SquareMatrix { rows, n: self.n }
    }
}

impl<T> MulIdentity for SquareMatrix<T>
where
    T: Add<Output = T> + Mul<Output = T> + AddIdentity + Copy + MulIdentity,
{
    fn mul_identity(&self) -> SquareMatrix<T> {
        let mut rows = vec![vec![self.rows[0][0].add_identity(); self.n]; self.n];
        for i in 0..self.n {
            rows[i][i] = self.rows[0][0].mul_identity();
        }
        SquareMatrix { rows, n: self.n }
    }
}

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
