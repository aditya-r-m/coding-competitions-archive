use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::algorithms;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModInt {
    pub i: usize,
    pub m: usize,
}

impl ModInt {
    pub fn new(i: usize, m: usize) -> ModInt {
        ModInt { i: i % m, m }
    }

    fn inv(&self) -> ModInt {
        assert!(self.i != 0);
        assert!(algorithms::is_probable_prime(self.m));
        let mut p0 = 0;
        let mut d0 = self.m;
        let mut p1 = 1;
        let mut d1 = self.i;
        while d1 > 1 {
            let r = d0 / d1;
            [p0, d0, p1, d1] = [
                p1,
                d1,
                (self.m + p0 - (r * p1) % self.m) % self.m,
                (self.m + d0 - (r * d1) % self.m) % self.m,
            ];
        }
        ModInt::new(p1, self.m)
    }
}

impl Neg for ModInt {
    type Output = ModInt;
    fn neg(self) -> ModInt {
        ModInt::new(self.m - self.i, self.m)
    }
}

impl Add for ModInt {
    type Output = ModInt;
    fn add(self, rhs: ModInt) -> ModInt {
        assert!(self.m == rhs.m);
        ModInt::new((self.i + rhs.i) % self.m, self.m)
    }
}

impl AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = ModInt::add(*self, rhs);
    }
}

impl Sub for ModInt {
    type Output = ModInt;
    fn sub(self, rhs: Self) -> ModInt {
        ModInt::add(self, rhs.neg())
    }
}

impl SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: Self) {
        *self = ModInt::sub(*self, rhs);
    }
}

impl Mul for ModInt {
    type Output = ModInt;
    fn mul(self, rhs: ModInt) -> ModInt {
        assert!(self.m == rhs.m);
        ModInt::new((self.i * rhs.i) % self.m, self.m)
    }
}

impl MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: Self) {
        *self = ModInt::mul(*self, rhs);
    }
}

impl Div for ModInt {
    type Output = ModInt;
    fn div(self, rhs: ModInt) -> ModInt {
        ModInt::mul(self, rhs.inv())
    }
}

impl DivAssign for ModInt {
    fn div_assign(&mut self, rhs: Self) {
        *self = ModInt::div(*self, rhs);
    }
}

pub trait AddIdentity {
    fn add_identity(&self) -> Self;
}

impl AddIdentity for f64 {
    fn add_identity(&self) -> Self {
        0.
    }
}

impl AddIdentity for ModInt {
    fn add_identity(&self) -> Self {
        ModInt::new(0, self.m)
    }
}

pub trait MulIdentity {
    fn mul_identity(&self) -> Self;
}

impl MulIdentity for f64 {
    fn mul_identity(&self) -> Self {
        1.
    }
}

impl MulIdentity for ModInt {
    fn mul_identity(&self) -> Self {
        ModInt::new(1, self.m)
    }
}

#[derive(Clone, Debug)]
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
    fn mul(self, rhs: SquareMatrix<T>) -> SquareMatrix<T> {
        assert!(self.n == rhs.n);
        let mut rows = vec![vec![self.rows[0][0].add_identity(); self.n]; self.n];
        for i in 0..self.n {
            for j in 0..self.n {
                for k in 0..self.n {
                    rows[i][j] = rows[i][j] + self.rows[i][k] * rhs.rows[k][j];
                }
            }
        }
        SquareMatrix { rows, n: self.n }
    }
}

impl<T> Mul<Vec<T>> for SquareMatrix<T>
where
    T: Add<Output = T> + Mul<Output = T> + AddIdentity + Copy,
{
    type Output = Vec<T>;
    fn mul(self, rhs: Vec<T>) -> Vec<T> {
        assert!(self.n == rhs.len());
        let mut result = vec![self.rows[0][0].add_identity(); self.n];
        for i in 0..self.n {
            for j in 0..self.n {
                result[i] = result[i] + self.rows[i][j] * rhs[j];
            }
        }
        result
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

#[derive(Debug)]
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
        if u == v {
            return;
        }
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

#[derive(Debug)]
pub struct BinaryIndexedTree<T>
where
    T: AddAssign + Copy,
{
    values: Vec<T>,
}

impl<T> BinaryIndexedTree<T>
where
    T: AddAssign + Copy,
{
    pub fn new(value: T, n: usize) -> BinaryIndexedTree<T> {
        BinaryIndexedTree {
            values: vec![value; n],
        }
    }

    pub fn add(&mut self, mut i: usize, v: T) {
        if i == 0 {
            self.values[0] += v;
            return;
        }
        while i < self.values.len() {
            self.values[i] += v;
            i += i & (!i + 1);
        }
    }

    pub fn get(&self, mut i: usize) -> T {
        let mut result = self.values[0];
        while i > 0 {
            result += self.values[i];
            i -= i & (!i + 1);
        }
        result
    }
}
