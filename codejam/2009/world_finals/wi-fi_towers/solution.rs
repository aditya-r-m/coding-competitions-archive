use std::collections::HashMap;

lib::run!();

fn read() -> Vec<[i64; 4]> {
    let mut points: Vec<[i64; 4]> = Vec::new();
    lib::input!(usize as n);
    for _ in 0..n {
        lib::input!([i64; 4] as point);
        points.push(point);
    }
    points
}

fn solve(points: Vec<[i64; 4]>) -> String {
    let source = points.len();
    let sink = source + 1;
    let mut graph: Vec<HashMap<usize, usize>> = vec![HashMap::new(); points.len() + 2];
    let mut total_positive_score = 0usize;
    for (p, &[px, py, r, score]) in points.iter().enumerate() {
        if score > 0 {
            total_positive_score += score as usize;
            graph[p].insert(sink, score as usize);
        } else {
            graph[source].insert(p, -score as usize);
        }
        for (q, &[qx, qy, _, _]) in points.iter().enumerate() {
            if q == p {
                continue;
            }
            if (px - qx).pow(2) + (py - qy).pow(2) <= r.pow(2) {
                graph[q].insert(p, 1_000_000_usize);
            }
        }
    }
    format!(
        "{}",
        total_positive_score - lib::algorithms::maximum_flow(&graph, source, sink)
    )
}
