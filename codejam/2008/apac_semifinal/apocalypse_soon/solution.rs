lib::run!();

const L: usize = 52;

struct TestCase {
    grid: [[usize; L]; L],
    r: usize,
    c: usize,
}

fn read() -> TestCase {
    let mut grid = [[0usize; L]; L];
    lib::input!([usize; 4] as [_, l, c, r]);
    for grid_row in grid.iter_mut().skip(1).take(l) {
        lib::input!(Vec<usize> as row);
        for (j, s) in row.into_iter().enumerate() {
            grid_row[j + 1] = s;
        }
    }
    TestCase { grid, r, c }
}

fn solve(TestCase { grid, r, c }: TestCase) -> String {
    if let Some(d) = dfs(grid, r, c) {
        format!("{} day(s)", d - 1)
    } else {
        "forever".to_string()
    }
}

fn dfs(grid: [[usize; L]; L], r: usize, c: usize) -> Option<usize> {
    if grid[r][c] == 0 {
        return Some(0);
    }
    if grid[r - 1][c] == 0 && grid[r][c - 1] == 0 && grid[r][c + 1] == 0 && grid[r + 1][c] == 0 {
        return None;
    }
    let mut next_grid = grid;
    for i in 0..L {
        for j in 0..L {
            if grid[i][j] == 0 || (i == r && j == c) {
                continue;
            }
            let mut target = (i - 1, j);
            for potential_target in [(i, j - 1), (i, j + 1), (i + 1, j)] {
                if grid[potential_target.0][potential_target.1] > grid[target.0][target.1] {
                    target = potential_target;
                }
            }
            next_grid[target.0][target.1] -=
                std::cmp::min(next_grid[target.0][target.1], grid[i][j]);
        }
    }
    if let Some(mut d) = dfs(next_grid, r, c) {
        for target in [(r - 1, c), (r, c - 1), (r, c + 1), (r + 1, c)] {
            let cached_value = next_grid[target.0][target.1];
            next_grid[target.0][target.1] -=
                std::cmp::min(next_grid[target.0][target.1], grid[r][c]);
            if let Some(s) = dfs(next_grid, r, c) {
                d = std::cmp::max(d, s);
            } else {
                return None;
            }
            next_grid[target.0][target.1] = cached_value;
        }
        Some(1 + d)
    } else {
        None
    }
}
