lib::run!();

#[derive(Clone, Copy)]
enum Node {
    LAND,
    RIVER,
    FOREST,
}

impl Node {
    fn new(c: char) -> Node {
        match c {
            '#' => Node::LAND,
            '.' => Node::RIVER,
            'T' => Node::FOREST,
            _ => panic!("Invalid Input"),
        }
    }
}

struct TestCase {
    grid: Vec<Vec<Node>>,
}

fn read() -> TestCase {
    let mut grid: Vec<Vec<Node>> = Vec::new();
    lib::input!([usize; 2] as [n, _]);
    for _ in 0..n {
        lib::input!(Vec<char> as row);
        grid.push(row.into_iter().map(Node::new).collect::<Vec<Node>>());
    }
    TestCase { grid }
}

fn solve(TestCase { grid }: TestCase) -> String {
    let n = grid.len();
    let m = grid[0].len();
    let flat_grid: Vec<Node> = grid.concat();
    let mut forests: Vec<usize> = Vec::new();
    let mut base_cost: Vec<usize> = vec![0; n * m];
    for v in 0..n * m {
        match flat_grid[v] {
            Node::FOREST => {
                forests.push(v);
            }
            Node::LAND => {
                base_cost[v] = usize::MAX;
            },
            _ => {}
        }
    }
    let mut forest_edge_list: Vec<(usize, (usize, usize))> = Vec::new();
    let forest_edge_cost =
        |d: usize| ((d * (d + 1)) >> 1) - ((d >> 1) * ((d >> 1) + 1)) + ((1 - (d & 1)) * (d >> 1));
    for f in forests.into_iter() {
        let mut visited: Vec<bool> = vec![false; n * m];
        let mut current_layer: Vec<usize> = vec![f];
        let mut current_distance = 0usize;
        while !current_layer.is_empty() {
            let mut next_layer: Vec<usize> = Vec::new();
            for g in current_layer.into_iter() {
                if visited[g] {
                    continue;
                }
                visited[g] = true;
                match flat_grid[g] {
                    Node::FOREST => {
                        if f < g {
                            forest_edge_list.push((forest_edge_cost(current_distance), (f, g)));
                        }
                    }
                    Node::RIVER => {
                        continue;
                    }
                    Node::LAND => {
                        base_cost[g] = std::cmp::min(base_cost[g], current_distance);
                    }
                }
                if g % m > 0 {
                    next_layer.push(g - 1);
                }
                if g % m + 1 < m {
                    next_layer.push(g + 1);
                }
                if g > m {
                    next_layer.push(g - m);
                }
                if g + m < n * m {
                    next_layer.push(g + m);
                }
            }
            current_layer = next_layer;
            current_distance += 1;
        }
    }
    let mut cost = base_cost.iter().sum::<usize>();
    let mut forest_disjoint_set = lib::collections::DisjointSet::new(n * m);
    forest_edge_list.sort();
    for (c, (f, g)) in forest_edge_list.into_iter() {
        if forest_disjoint_set.get_root(f) == forest_disjoint_set.get_root(g) {
            continue;
        }
        forest_disjoint_set.union(f, g);
        cost += c;
    }
    format!("{cost}")
}
