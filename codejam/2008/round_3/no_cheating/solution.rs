use std::collections::HashMap;

lib::run!();

fn read() -> Vec<Vec<char>> {
    lib::input!([usize; 2] as [n, _]);
    let mut grid: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        lib::input!(Vec<char> as row);
        grid.push(row);
    }
    grid
}

fn solve(grid: Vec<Vec<char>>) -> String {
    format!(
        "{}",
        grid.iter()
            .map(|row| {
                row.iter()
                    .map(|&c| if c == '.' { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum::<usize>()
            - {
                let (graph, s, t) = parse(grid);
                lib::algorithms::maximum_flow(&graph, s, t)
            }
    )
}

fn parse(mut grid: Vec<Vec<char>>) -> (Vec<HashMap<usize, usize>>, usize, usize) {
    let nr = grid.len();
    let nc = grid[0].len() + (grid[0].len() & 1);
    if nc != grid[0].len() {
        for row in grid.iter_mut() {
            row.push('x');
        }
    }

    let mut graph: Vec<HashMap<usize, usize>> = vec![HashMap::new(); nr * nc];
    let norm = |i, j| i * nc + j;
    for i in 0..nr {
        for j in 1..nc {
            if grid[i][j] != '.' {
                continue;
            }
            for ni in if i > 0 { i - 1 } else { 0 }..std::cmp::min(i + 2, grid.len()) {
                if grid[ni][j - 1] == '.' {
                    graph[norm(i, j)].insert(norm(ni, j - 1), 1);
                    graph[norm(ni, j - 1)].insert(norm(i, j), 1);
                }
            }
        }
    }
    let n = graph.len();
    let s = graph.len();
    graph.push(HashMap::new());
    let t = graph.len();
    graph.push(HashMap::new());
    for u in 0..n {
        if u & 1 == 0 {
            graph[s].insert(u, 1);
        } else {
            graph[u].clear();
            graph[u].insert(t, 1);
        }
    }
    (graph, s, t)
}
