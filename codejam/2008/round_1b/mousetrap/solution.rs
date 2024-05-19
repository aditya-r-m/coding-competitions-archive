lib::run!();

struct TestCase {
    k: usize,
    queries: Vec<usize>,
}

fn read() -> TestCase {
    lib::input!(usize as k);
    lib::input!(Vec<usize> as queries);
    queries.remove(0);
    TestCase { k, queries }
}

struct SegmentTree {
    range: Vec<[usize; 2]>,
    space: Vec<usize>,
    value: Vec<usize>,
}

impl SegmentTree {
    fn new(k: usize) -> SegmentTree {
        let mut segment_tree = SegmentTree {
            range: vec![[0, 0]; k << 2],
            space: vec![0; k << 2],
            value: vec![0; k << 2],
        };
        segment_tree.init(0, 0, k - 1);
        segment_tree
    }

    fn init(&mut self, i: usize, l: usize, r: usize) {
        self.space[i] = 1 + r - l;
        self.range[i] = [l, r];
        if l < r {
            self.init((i << 1) + 1, l, l + ((r - l) >> 1));
            self.init((i + 1) << 1, 1 + l + ((r - l) >> 1), r);
        }
    }

    fn insert(&mut self, i: usize, mut key: usize, val: usize) {
        self.space[i] -= 1;
        let [l, r] = self.range[i];
        if l == r {
            self.value[i] = val;
        } else if self.space[(i << 1) + 1] > key {
            self.insert((i << 1) + 1, key, val);
        } else {
            key -= self.space[(i << 1) + 1];
            self.insert((i + 1) << 1, key, val);
        }
    }

    fn query(&self, i: usize, key: usize) -> usize {
        let [l, r] = self.range[i];
        if l == r {
            self.value[i]
        } else if self.range[(i << 1) + 1][1] >= key {
            self.query((i << 1) + 1, key)
        } else {
            self.query((i + 1) << 1, key)
        }
    }
}

fn solve(TestCase { k, queries }: TestCase) -> String {
    let mut segment_tree = SegmentTree::new(k);
    let mut c = 0usize;
    for i in 0..k {
        c += i;
        c %= k - i;
        segment_tree.insert(0, c, i + 1);
    }
    queries
        .into_iter()
        .map(|i| format!("{}", segment_tree.query(0, i - 1)))
        .collect::<Vec<String>>()
        .join(" ")
}
