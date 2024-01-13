use lib::collections::{SegmentDirection, SegmentTree};
use std::collections::HashMap;

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

fn solve(TestCase { k, queries }: TestCase) -> String {
    let mut segment_tree: SegmentTree<usize, usize> = SegmentTree::create(
        0,
        k - 1,
        |root, left, right| {
            if root.min == root.max {
                if root.value.is_none() {
                    Some(1)
                } else {
                    Some(0)
                }
            } else {
                Some(
                    SegmentTree::annotation_or(left, root.left_max - root.min + 1)
                        + SegmentTree::annotation_or(right, root.max - root.right_min + 1),
                )
            }
        },
        |min, _, value, root, left, _| {
            if root.min == root.max {
                HashMap::from([(SegmentDirection::Term, (min, min, value))])
            } else {
                let left_space = SegmentTree::annotation_or(left, root.left_max - root.min + 1);
                if left_space > min {
                    HashMap::from([(SegmentDirection::Left, (min, min, value))])
                } else {
                    HashMap::from([(
                        SegmentDirection::Right,
                        (min - left_space, min - left_space, value),
                    )])
                }
            }
        },
        |min, _, root, _, _| {
            if root.min == root.max {
                (SegmentDirection::Term, min, min)
            } else if min <= root.left_max {
                (SegmentDirection::Left, min, min)
            } else {
                (SegmentDirection::Right, min, min)
            }
        },
    );

    let mut c = 0usize;
    for i in 0..k {
        c += i;
        c %= k - i;
        segment_tree.insert(c, c, Some(i + 1));
    }

    queries
        .into_iter()
        .map(|i| format!("{}", segment_tree.query(i - 1, i - 1).unwrap()))
        .collect::<Vec<String>>()
        .join(" ")
}
