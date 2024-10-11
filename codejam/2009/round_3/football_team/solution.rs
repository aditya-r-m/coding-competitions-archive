use std::collections::{BTreeMap, BTreeSet};

lib::run!();

fn read() -> Vec<BTreeSet<usize>> {
    lib::input!(usize as n);
    let mut graph: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];
    let mut points: BTreeMap<[usize; 2], usize> = BTreeMap::new();
    for i in 0..n {
        lib::input!([usize; 2] as point);
        points.insert(point, i);
    }
    for (point, &i) in points.iter() {
        for y in point[1] - 1..=point[1] + 1 {
            for x in point[0] + 1..1001 {
                if let Some(&j) = points.get(&[x, y]) {
                    graph[i].insert(j);
                    graph[j].insert(i);
                    break;
                }
            }
        }
    }
    graph
}

fn solve(graph: Vec<BTreeSet<usize>>) -> String {
    format!(
        "{}",
        if graph.iter().all(|s| s.len() == 0) {
            1
        } else if graph
            .iter()
            .all(|s| s.iter().all(|j| s.iter().all(|&k| !graph[k].contains(j))))
        {
            2
        } else if graph.iter().all(|s| {
            s.len() & 1 == 0
                || s.iter().any(|j| {
                    2 > s
                        .iter()
                        .map(|&k| if graph[k].contains(j) { 1 } else { 0 })
                        .sum()
                })
        }) {
            3
        } else {
            4
        }
    )
}
