use std::collections::HashMap;

lib::run!();

fn read() -> Vec<Vec<usize>> {
    let mut charts: Vec<Vec<usize>> = Vec::new();
    lib::input!([usize; 2] as [n, _]);
    for _ in 0..n {
        lib::input!(Vec<usize> as chart);
        charts.push(chart);
    }
    charts
}

fn solve(charts: Vec<Vec<usize>>) -> String {
    let n = charts.len();
    let mut graph: Vec<HashMap<usize, usize>> = vec![HashMap::new(); 2 * n + 2];
    let s = 2 * n;
    let t = 2 * n + 1;
    for i in 0..n {
        graph[s].insert(i, 1);
        graph[n + i].insert(t, 1);
    }
    for (u, chart_u) in charts.iter().enumerate() {
        for (v, chart_v) in charts.iter().enumerate() {
            if chart_u.iter().zip(chart_v.iter()).all(|(pu, pv)| pu > pv) {
                graph[u].insert(n + v, 1);
            }
        }
    }
    format!("{}", n - lib::algorithms::maximum_flow(&graph, s, t))
}
