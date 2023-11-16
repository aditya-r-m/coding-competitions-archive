use std::cmp::min;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::stdin;

fn parse(mut grid: Vec<Vec<char>>) -> Vec<BTreeSet<usize>> {
    let nr = grid.len();
    let nc = grid[0].len() + (grid[0].len() & 1);
    if nc != grid[0].len() {
        for i in 0..nr {
            grid[i].push('x');
        }
    }

    let mut graph: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); nr * nc];
    let norm = |i, j| i * nc + j;
    for i in 0..nr {
        for j in 1..nc {
            if grid[i][j] != '.' {
                continue;
            }
            for ni in if i > 0 { i - 1 } else { 0 }..min(i + 2, grid.len()) {
                if grid[ni][j - 1] == '.' {
                    graph[norm(i, j)].insert(norm(ni, j - 1));
                    graph[norm(ni, j - 1)].insert(norm(i, j));
                }
            }
        }
    }
    graph
}

fn maximum_flow(mut graph: Vec<BTreeSet<usize>>) -> usize {
    let n = graph.len();
    let s = graph.len();
    graph.push(BTreeSet::new());
    let t = graph.len();
    graph.push(BTreeSet::new());
    for u in 0..n {
        if u & 1 == 0 {
            graph[s].insert(u);
        } else {
            graph[u].clear();
            graph[u].insert(t);
        }
    }

    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut path: HashMap<usize, usize> = HashMap::new();
    let mut augmenting_path_found = true;
    while augmenting_path_found {
        augmenting_path_found = false;
        queue.clear();
        queue.push_back(s);
        path.clear();
        path.insert(s, s);

        while let Some(mut u) = queue.pop_front() {
            if u == t {
                augmenting_path_found = true;
                while u != s {
                    graph[u].insert(path[&u]);
                    graph[path[&u]].remove(&u);
                    u = path[&u];
                }
                break;
            }
            for &v in graph[u].iter() {
                if !path.contains_key(&v) {
                    queue.push_back(v);
                    path.insert(v, u);
                }
            }
        }
    }
    graph[t].len()
}

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let test_case_count = buffer.trim().parse::<usize>().unwrap();
    buffer.clear();

    for test_case_index in 1..1 + test_case_count {
        stdin().read_line(&mut buffer).unwrap();
        let n = buffer
            .split(" ")
            .next()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        buffer.clear();

        let mut grid: Vec<Vec<char>> = Vec::new();
        for _ in 0..n {
            stdin().read_line(&mut buffer).unwrap();
            grid.push(buffer.trim().chars().collect::<Vec<char>>());
            buffer.clear();
        }

        let solution = grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&c| if c == '.' { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum::<usize>()
            - maximum_flow(parse(grid));
        println!("Case #{test_case_index}: {solution}");
    }
}
