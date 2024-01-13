use std::collections::HashMap;
use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug)]
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

#[derive(Debug)]
pub struct SegmentNode<V, A> {
    pub min: usize,
    pub max: usize,
    pub left_max: usize,
    pub right_min: usize,
    pub value: Option<V>,
    pub annotation: Option<A>,
}

#[derive(PartialEq, Eq, Hash)]
pub enum SegmentDirection {
    Term,
    Left,
    Right,
}

pub type SegmentAnnotator<V, A> =
    fn(&SegmentNode<V, A>, Option<&SegmentNode<V, A>>, Option<&SegmentNode<V, A>>) -> Option<A>;

pub type SegmentPropogator<V, A> = fn(
    min: usize,
    max: usize,
    value: Option<V>,
    &SegmentNode<V, A>,
    Option<&SegmentNode<V, A>>,
    Option<&SegmentNode<V, A>>,
) -> HashMap<SegmentDirection, (usize, usize, Option<V>)>;

pub type SegmentNavigator<V, A> = fn(
    min: usize,
    max: usize,
    &SegmentNode<V, A>,
    Option<&SegmentNode<V, A>>,
    Option<&SegmentNode<V, A>>,
) -> (SegmentDirection, usize, usize);

#[derive(Debug)]
pub struct SegmentTree<V, A> {
    node: SegmentNode<V, A>,
    left_subtree: Option<Box<SegmentTree<V, A>>>,
    right_subtree: Option<Box<SegmentTree<V, A>>>,
    annotator: SegmentAnnotator<V, A>,
    propogator: SegmentPropogator<V, A>,
    navigator: SegmentNavigator<V, A>,
}

impl<V, A> SegmentTree<V, A>
where
    V: Copy,
    A: Copy,
{
    pub fn create(
        min: usize,
        max: usize,
        annotator: SegmentAnnotator<V, A>,
        propogator: SegmentPropogator<V, A>,
        navigator: SegmentNavigator<V, A>,
    ) -> SegmentTree<V, A> {
        let mut segment_tree = SegmentTree {
            node: SegmentNode {
                min,
                max,
                left_max: min + ((max - min) >> 1),
                right_min: std::cmp::min(max, 1 + min + ((max - min) >> 1)),
                value: None,
                annotation: None,
            },
            left_subtree: None,
            right_subtree: None,
            annotator,
            propogator,
            navigator,
        };
        segment_tree.annotate();
        segment_tree
    }

    fn left_node(&self) -> Option<&SegmentNode<V, A>> {
        if let Some(l) = self.left_subtree.as_ref() {
            Some(&l.node)
        } else {
            None
        }
    }

    fn right_node(&self) -> Option<&SegmentNode<V, A>> {
        if let Some(r) = self.right_subtree.as_ref() {
            Some(&r.node)
        } else {
            None
        }
    }

    pub fn annotation_or(node: Option<&SegmentNode<V, A>>, a: A) -> A {
        if let Some(n) = node {
            n.annotation.unwrap_or(a)
        } else {
            a
        }
    }

    fn touch_left(&mut self) {
        if self.left_subtree.is_none() {
            self.left_subtree = Some(Box::new(Self::create(
                self.node.min,
                self.node.left_max,
                self.annotator,
                self.propogator,
                self.navigator,
            )));
        }
    }

    fn touch_right(&mut self) {
        if self.right_subtree.is_none() {
            self.right_subtree = Some(Box::new(Self::create(
                self.node.right_min,
                self.node.max,
                self.annotator,
                self.propogator,
                self.navigator,
            )));
        }
    }

    fn annotate(&mut self) {
        self.node.annotation = (self.annotator)(&self.node, self.left_node(), self.right_node());
    }

    pub fn insert(&mut self, min: usize, max: usize, value: Option<V>) {
        let scope = (self.propogator)(
            min,
            max,
            value,
            &self.node,
            self.left_node(),
            self.right_node(),
        );
        if let Some(&(_, _, value)) = scope.get(&SegmentDirection::Term) {
            self.node.value = value;
        }
        if let Some(&(min, max, value)) = scope.get(&SegmentDirection::Left) {
            self.touch_left();
            self.left_subtree.as_mut().unwrap().insert(min, max, value);
        }
        if let Some(&(min, max, value)) = scope.get(&SegmentDirection::Right) {
            self.touch_right();
            self.right_subtree.as_mut().unwrap().insert(min, max, value);
        }
        self.annotate()
    }

    pub fn query(&self, min: usize, max: usize) -> Option<V> {
        match (self.navigator)(min, max, &self.node, self.left_node(), self.right_node()) {
            (SegmentDirection::Term, _, _) => self.node.value,
            (SegmentDirection::Left, min, max) => {
                if self.left_subtree.is_none() {
                    None
                } else {
                    self.left_subtree.as_ref().unwrap().query(min, max)
                }
            }
            (SegmentDirection::Right, min, max) => {
                if self.right_subtree.is_none() {
                    None
                } else {
                    self.right_subtree.as_ref().unwrap().query(min, max)
                }
            }
        }
    }
}
