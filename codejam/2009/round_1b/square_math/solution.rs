use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Result};

lib::run!();

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Cell {
    SGN(bool),
    VAL(i64),
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Cell::SGN(s) = self {
            write!(f, "{}", if *s { "-" } else { "+" })?;
        }
        if let Cell::VAL(v) = self {
            write!(f, "{v}")?;
        }
        Result::Ok(())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Path {
    val: i64,
    len: usize,
    cells: Vec<Cell>,
    last: (usize, usize),
}

impl Path {
    fn new(val: i64, last: (usize, usize)) -> Path {
        Path {
            val,
            len: 1,
            cells: vec![Cell::VAL(val)],
            last,
        }
    }
    fn push(&mut self, sgn: bool, val: i64, last: (usize, usize)) {
        self.val += val * if sgn { -1 } else { 1 };
        self.len += 2;
        self.cells.push(Cell::SGN(sgn));
        self.cells.push(Cell::VAL(val));
        self.last = last;
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for cell in self.cells.iter() {
            write!(f, "{cell}")?;
        }
        Result::Ok(())
    }
}

struct TestCase {
    grid: Vec<Vec<Cell>>,
    queries: Vec<i64>,
}

fn read() -> TestCase {
    lib::input!([usize; 2] as [w, _]);
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    for _ in 0..w {
        lib::input!(Vec<char> as row);
        grid.push(
            row.into_iter()
                .map(|c| match c {
                    '+' => Cell::SGN(false),
                    '-' => Cell::SGN(true),
                    _ => Cell::VAL(c.to_digit(10).unwrap() as i64),
                })
                .collect(),
        );
    }
    lib::input!(Vec<i64> as queries);
    TestCase { grid, queries }
}

fn solve(TestCase { grid, queries }: TestCase) -> String {
    let mut query_set: HashSet<i64> = HashSet::from_iter(queries.iter().cloned());
    let mut cur_paths: Vec<Path> = Vec::new();
    let mut nxt_paths: Vec<Path> = Vec::new();
    let mut result_map: HashMap<i64, Path> = HashMap::new();
    for i in 1..=grid.len() {
        for j in 1..=grid.len() {
            if let Cell::VAL(v) = grid[i - 1][j - 1] {
                cur_paths.push(Path::new(v, (i, j)));
            }
        }
    }
    let mut visited: HashSet<(i64, (usize, usize))> = HashSet::new();
    while !query_set.is_empty() {
        cur_paths.sort();
        nxt_paths.clear();
        for cur_path in cur_paths.iter() {
            if visited.contains(&(cur_path.val, cur_path.last)) {
                continue;
            }
            visited.insert((cur_path.val, cur_path.last));
            query_set.remove(&cur_path.val);
            result_map.entry(cur_path.val).or_insert(cur_path.clone());
            let (i, j) = cur_path.last;
            for (m0, n0) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)].into_iter() {
                for (m1, n1) in [(m0 - 1, n0), (m0 + 1, n0), (m0, n0 - 1), (m0, n0 + 1)].into_iter()
                {
                    if [m0, n0, m1, n1].into_iter().reduce(std::cmp::min).unwrap() == 0
                        || [m0, n0, m1, n1].into_iter().reduce(std::cmp::max).unwrap() > grid.len()
                    {
                        continue;
                    }
                    let mut nxt_path = cur_path.clone();
                    if let Cell::SGN(s) = grid[m0 - 1][n0 - 1] {
                        if let Cell::VAL(v) = grid[m1 - 1][n1 - 1] {
                            nxt_path.push(s, v, (m1, n1));
                        }
                    }
                    nxt_paths.push(nxt_path);
                }
            }
        }
        (cur_paths, nxt_paths) = (nxt_paths, cur_paths);
    }
    format!(
        "{}",
        queries
            .into_iter()
            .map(|q| format!("\n{}", result_map.get(&q).unwrap()))
            .collect::<String>()
    )
}
