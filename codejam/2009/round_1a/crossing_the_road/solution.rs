lib::run!();

fn read() -> Vec<Vec<[usize; 3]>> {
    let mut grid: Vec<Vec<[usize; 3]>> = Vec::new();
    lib::input!([usize; 2] as [n, _]);
    for _ in 0..n {
        lib::input!(Vec<usize> as row);
        grid.push(
            row.chunks(3)
                .map(|p| [p[0], p[1], p[2]])
                .collect::<Vec<[usize; 3]>>(),
        );
    }
    grid
}

fn solve(grid: Vec<Vec<[usize; 3]>>) -> String {
    let (graph, (source, target)) = Graph::parse(&grid);
    let mut queue = std::collections::BinaryHeap::<(std::cmp::Reverse<usize>, usize)>::new();
    let mut visited = std::collections::HashSet::<usize>::new();
    queue.push((std::cmp::Reverse(0), source));
    loop {
        let (std::cmp::Reverse(c), u) = queue.pop().unwrap();
        if visited.contains(&u) {
            continue;
        }
        visited.insert(u);
        if u == target {
            return format!("{}", c);
        }
        for (v, d) in graph.get_neighbors(u, c) {
            queue.push((std::cmp::Reverse(d), v));
        }
    }
}

#[derive(Copy, Clone)]
enum Distance {
    STATIC,
    DYNAMIC(i64, i64, i64),
}

struct Graph {
    adjacency_list: Vec<Vec<(usize, Distance)>>,
}

impl Graph {
    fn parse(grid: &Vec<Vec<[usize; 3]>>) -> (Graph, (usize, usize)) {
        let n = grid.len();
        let m = grid[0].len();
        let mut graph = Graph {
            adjacency_list: vec![Vec::new(); 4 * n * m],
        };
        let serialize = |r: usize, c: usize| 2 * m * r + c;
        let mut add_edge = |(r0, c0): (usize, usize), (r1, c1): (usize, usize), d: Distance| {
            graph.adjacency_list[serialize(r0, c0)].push((serialize(r1, c1), d));
            graph.adjacency_list[serialize(r1, c1)].push((serialize(r0, c0), d));
        };
        for i in 0..n {
            for j in 0..m {
                if i < n - 1 {
                    add_edge((2 * i + 1, 2 * j), (2 * (i + 1), 2 * j), Distance::STATIC);
                    add_edge(
                        (2 * i + 1, 2 * j + 1),
                        (2 * (i + 1), 2 * j + 1),
                        Distance::STATIC,
                    );
                }
                if j < m - 1 {
                    add_edge((2 * i, 2 * j + 1), (2 * i, 2 * (j + 1)), Distance::STATIC);
                    add_edge(
                        (2 * i + 1, 2 * j + 1),
                        (2 * i + 1, 2 * (j + 1)),
                        Distance::STATIC,
                    );
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                let [s, w, t] = grid[i][j];
                add_edge(
                    (2 * i, 2 * j),
                    (2 * i + 1, 2 * j),
                    Distance::DYNAMIC((s + w) as i64, w as i64, (t + s) as i64),
                );
                add_edge(
                    (2 * i, 2 * j + 1),
                    (2 * i + 1, 2 * j + 1),
                    Distance::DYNAMIC((s + w) as i64, w as i64, (t + s) as i64),
                );
                add_edge(
                    (2 * i, 2 * j),
                    (2 * i, 2 * j + 1),
                    Distance::DYNAMIC((s + w) as i64, s as i64, t as i64),
                );
                add_edge(
                    (2 * i + 1, 2 * j),
                    (2 * i + 1, 2 * j + 1),
                    Distance::DYNAMIC((s + w) as i64, s as i64, t as i64),
                );
            }
        }
        (graph, (serialize(2 * n - 1, 0), serialize(0, 2 * m - 1)))
    }
    fn get_neighbors(&self, u: usize, t: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        for &(v, d) in self.adjacency_list[u].iter() {
            if let Distance::DYNAMIC(duration, wait_time, offset) = d {
                neighbors.push((
                    v,
                    t + 1
                        + std::cmp::max(0, wait_time - (t as i64 - offset).rem_euclid(duration))
                            as usize,
                ));
            } else {
                neighbors.push((v, t + 2));
            }
        }
        neighbors
    }
}
