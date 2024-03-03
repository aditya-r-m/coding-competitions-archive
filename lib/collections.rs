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

impl AddIdentity for ModInt {
    fn add_identity(&self) -> Self {
        ModInt::new(0, self.m)
    }
}

pub trait MulIdentity {
    fn mul_identity(&self) -> Self;
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

#[derive(Debug)]
pub struct SegmentNode<S> {
    pub min: usize,
    pub mid: usize,
    pub max: usize,
    pub state: S,
}

pub type StateInitializer<S> = fn(usize, usize, usize) -> S;

pub type UpdatePropagator<S, U> = fn(&SegmentNode<S>, U) -> (Option<U>, Option<U>);

pub type UpdateAggregator<S, U> = fn(&SegmentNode<S>, U, Option<S>, Option<S>) -> S;

pub type QueryPropagator<S, Q> = fn(&SegmentNode<S>, Q) -> (Option<Q>, Option<Q>);

pub type QueryAggregator<S, Q, R> = fn(&SegmentNode<S>, Q, Option<R>, Option<R>) -> R;

pub struct SegmentTree<S, U, Q, R> {
    node: SegmentNode<S>,
    left_subtree: Option<Box<SegmentTree<S, U, Q, R>>>,
    right_subtree: Option<Box<SegmentTree<S, U, Q, R>>>,
    state_initializer: StateInitializer<S>,
    update_propagator: UpdatePropagator<S, U>,
    update_aggregator: UpdateAggregator<S, U>,
    query_propagator: QueryPropagator<S, Q>,
    query_aggregator: QueryAggregator<S, Q, R>,
}

impl<S, U, Q, R> std::fmt::Debug for SegmentTree<S, U, Q, R>
where
    S: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.node.fmt(f)?;
        if let Some(ltree) = self.left_subtree.as_ref() {
            f.write_str("\n")?;
            ltree.fmt(f)?;
        }
        if let Some(rtree) = self.right_subtree.as_ref() {
            f.write_str("\n")?;
            rtree.fmt(f)?;
        }
        Result::Ok(())
    }
}

impl<S, U, Q, R> SegmentTree<S, U, Q, R>
where
    S: Copy,
    U: Copy,
    Q: Copy,
    R: Copy,
{
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        min: usize,
        max: usize,
        strict_initialization: bool,
        state_initializer: StateInitializer<S>,
        update_propagator: UpdatePropagator<S, U>,
        update_aggregator: UpdateAggregator<S, U>,
        query_propagator: QueryPropagator<S, Q>,
        query_aggregator: QueryAggregator<S, Q, R>,
    ) -> SegmentTree<S, U, Q, R> {
        let mid = min + ((max - min) >> 1);
        let mut tree = SegmentTree {
            node: SegmentNode {
                min,
                mid,
                max,
                state: state_initializer(min, mid, max),
            },
            left_subtree: None,
            right_subtree: None,
            state_initializer,
            update_propagator,
            update_aggregator,
            query_propagator,
            query_aggregator,
        };
        if strict_initialization && min < max {
            tree.touch_left(strict_initialization);
            tree.touch_right(strict_initialization);
        }
        tree
    }

    fn left_state(&self) -> Option<S> {
        self.left_subtree.as_ref().map(|l| l.node.state)
    }

    fn right_state(&self) -> Option<S> {
        self.right_subtree.as_ref().map(|r| r.node.state)
    }

    fn touch_left(&mut self, strict_initialization: bool) {
        if self.left_subtree.is_none() {
            self.left_subtree = Some(Box::new(Self::create(
                self.node.min,
                self.node.mid,
                strict_initialization,
                self.state_initializer,
                self.update_propagator,
                self.update_aggregator,
                self.query_propagator,
                self.query_aggregator,
            )));
        }
    }

    fn touch_right(&mut self, strict_initialization: bool) {
        if self.right_subtree.is_none() {
            self.right_subtree = Some(Box::new(Self::create(
                self.node.mid + 1,
                self.node.max,
                strict_initialization,
                self.state_initializer,
                self.update_propagator,
                self.update_aggregator,
                self.query_propagator,
                self.query_aggregator,
            )));
        }
    }

    pub fn update(&mut self, u: U) {
        let scope = (self.update_propagator)(&self.node, u);
        if let Some(lu) = scope.0 {
            self.touch_left(false);
            self.left_subtree.as_mut().unwrap().update(lu);
        }
        if let Some(ru) = scope.1 {
            self.touch_right(false);
            self.right_subtree.as_mut().unwrap().update(ru);
        }
        self.node.state =
            (self.update_aggregator)(&self.node, u, self.left_state(), self.right_state());
    }

    pub fn query(&self, q: Q) -> R {
        let scope = (self.query_propagator)(&self.node, q);
        (self.query_aggregator)(
            &self.node,
            q,
            scope
                .0
                .map(|q| self.left_subtree.as_ref().unwrap().query(q)),
            scope
                .1
                .map(|q| self.right_subtree.as_ref().unwrap().query(q)),
        )
    }
}
