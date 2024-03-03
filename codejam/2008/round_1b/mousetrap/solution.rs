use lib::collections::{SegmentNode, SegmentTree};

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

#[derive(Clone, Copy)]
struct S {
    value: usize,
    capacity_self: usize,
    capacity_left: usize,
}

#[derive(Clone, Copy)]
struct U {
    index: usize,
    value: usize,
}

fn state_initializer(min: usize, mid: usize, max: usize) -> S {
    S {
        value: 0,
        capacity_self: 1 + max - min,
        capacity_left: 1 + mid - min,
    }
}

fn update_propagator(node: &SegmentNode<S>, mut u: U) -> (Option<U>, Option<U>) {
    if node.min == node.max {
        (None, None)
    } else if u.index < node.state.capacity_left {
        (Some(u), None)
    } else {
        u.index -= node.state.capacity_left;
        (None, Some(u))
    }
}

fn update_aggregator(node: &SegmentNode<S>, u: U, l: Option<S>, r: Option<S>) -> S {
    if node.min == node.max {
        S {
            value: u.value,
            capacity_self: 0,
            capacity_left: 0,
        }
    } else {
        S {
            value: 0,
            capacity_self: l.unwrap().capacity_self + r.unwrap().capacity_self,
            capacity_left: l.unwrap().capacity_self,
        }
    }
}

fn query_propagator(node: &SegmentNode<S>, q: usize) -> (Option<usize>, Option<usize>) {
    if node.min == node.max {
        (None, None)
    } else if q <= node.mid {
        (Some(q), None)
    } else {
        (None, Some(q))
    }
}

fn query_aggregator(node: &SegmentNode<S>, _: usize, l: Option<usize>, r: Option<usize>) -> usize {
    if node.min == node.max {
        node.state.value
    } else {
        l.or(r).unwrap()
    }
}

fn solve(TestCase { k, queries }: TestCase) -> String {
    let mut segment_tree: SegmentTree<S, U, usize, usize> = SegmentTree::create(
        0,
        k - 1,
        true,
        state_initializer,
        update_propagator,
        update_aggregator,
        query_propagator,
        query_aggregator,
    );

    let mut c = 0usize;
    for i in 0..k {
        c += i;
        c %= k - i;
        segment_tree.update(U {
            index: c,
            value: i + 1,
        });
    }

    queries
        .into_iter()
        .map(|i| format!("{}", segment_tree.query(i - 1)))
        .collect::<Vec<String>>()
        .join(" ")
}
