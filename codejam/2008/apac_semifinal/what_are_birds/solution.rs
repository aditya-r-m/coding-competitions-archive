use std::cmp::{max, min};
use std::collections::HashSet;

lib::run!();

struct TestCase {
    bird_metrics: Vec<(usize, usize)>,
    not_bird_metrics: Vec<(usize, usize)>,
    query_metrics: Vec<(usize, usize)>,
}

fn read() -> TestCase {
    let mut bird_metrics: Vec<(usize, usize)> = Vec::new();
    let mut not_bird_metrics: Vec<(usize, usize)> = Vec::new();
    let mut query_metrics: Vec<(usize, usize)> = Vec::new();
    lib::input!(usize as n);
    for _ in 0..n {
        lib::input!((usize, usize, String) as (h, w, x));
        if x == "BIRD" {
            &mut bird_metrics
        } else {
            &mut not_bird_metrics
        }
        .push((h, w));
    }
    lib::input!(usize as m);
    for _ in 0..m {
        lib::input!([usize; 2] as [h, w]);
        query_metrics.push((h, w));
    }
    TestCase {
        bird_metrics,
        not_bird_metrics,
        query_metrics,
    }
}

fn solve(
    TestCase {
        bird_metrics,
        not_bird_metrics,
        query_metrics,
    }: TestCase,
) -> String {
    let result = if bird_metrics.is_empty() {
        let not_bird_metric_set: HashSet<(usize, usize)> = HashSet::from_iter(not_bird_metrics);
        query_metrics
            .into_iter()
            .map(|q| {
                if not_bird_metric_set.contains(&q) {
                    "NOT BIRD".to_owned()
                } else {
                    "UNKNOWN".to_owned()
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    } else {
        let min_bird = (
            bird_metrics.iter().map(|&r| r.0).reduce(min).unwrap(),
            bird_metrics.iter().map(|&r| r.1).reduce(min).unwrap(),
        );
        let max_bird = (
            bird_metrics.iter().map(|&r| r.0).reduce(max).unwrap(),
            bird_metrics.iter().map(|&r| r.1).reduce(max).unwrap(),
        );
        macro_rules! seq {
            ($i: tt, $a: ident, $b: ident, $c: ident) => {
                ($a.$i <= $b.$i && $b.$i <= $c.$i)
            };
        }
        query_metrics
            .into_iter()
            .map(|query| {
                if seq!(0, min_bird, query, max_bird) && seq!(1, min_bird, query, max_bird) {
                    "BIRD".to_owned()
                } else if not_bird_metrics.iter().any(|not_bird| {
                    [
                        (seq!(0, min_bird, not_bird, max_bird)
                            && (seq!(1, max_bird, not_bird, query)
                                || seq!(1, query, not_bird, min_bird))),
                        (seq!(1, min_bird, not_bird, max_bird)
                            && (seq!(0, max_bird, not_bird, query)
                                || seq!(0, query, not_bird, min_bird))),
                        (seq!(0, query, not_bird, min_bird) && seq!(1, query, not_bird, min_bird)),
                        (seq!(0, query, not_bird, min_bird) && seq!(1, min_bird, not_bird, query)),
                        (seq!(0, min_bird, not_bird, query) && seq!(1, query, not_bird, min_bird)),
                        (seq!(0, min_bird, not_bird, query) && seq!(1, min_bird, not_bird, query)),
                    ]
                    .into_iter()
                    .any(|i| i)
                }) {
                    "NOT BIRD".to_owned()
                } else {
                    "UNKNOWN".to_owned()
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    };
    format!("\n{result}")
}
