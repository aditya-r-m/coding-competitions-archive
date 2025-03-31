lib::run!();

struct TestCase {
    grid: Vec<Vec<bool>>,
    source: (usize, usize),
}

fn read() -> TestCase {
    lib::input!([usize; 2] as [row_count, _]);
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut source: (usize, usize) = (0, 0);
    for r in 0..row_count {
        lib::input!(row);
        grid.push(row.chars().map(|c| c != '#').collect::<Vec<bool>>());
        if let Some((c, _)) = row.chars().enumerate().find(|&(_, ch)| ch == 'K') {
            source = (r, c);
        }
    }
    TestCase { grid, source }
}

fn solve(TestCase { grid, source }: TestCase) -> String {
    if first_move_advantage(grid, source) {
        "A".to_string()
    } else {
        "B".to_string()
    }
}

fn first_move_advantage(grid: Vec<Vec<bool>>, source: (usize, usize)) -> bool {
    let (w, h) = (grid[0].len(), grid.len());
    let mut restricted_grid = grid.clone();
    restricted_grid[source.0][source.1] = false;

    maximum_matching(
        &mut vec![vec![vec![None; 1 << (w + 1)]; w]; h],
        &grid,
        0,
        0,
        0,
    ) > maximum_matching(
        &mut vec![vec![vec![None; 1 << (w + 1)]; w]; h],
        &restricted_grid,
        0,
        0,
        0,
    )
}

fn maximum_matching(
    cache: &mut Vec<Vec<Vec<Option<usize>>>>,
    grid: &Vec<Vec<bool>>,
    i: usize,
    j: usize,
    m: usize,
) -> usize {
    let (w, h) = (grid[0].len(), grid.len());
    if i + 1 == h && j + 1 == w {
        cache[i][j][m] = Some(0);
    }
    if let Some(v) = cache[i][j][m] {
        return v;
    }

    let nj = (j + 1) % w;
    let ni = i + if nj == 0 { 1 } else { 0 };
    let mut v = maximum_matching(cache, grid, ni, nj, m >> 1);
    if grid[i][j] && m & 1 == 0 {
        if j + 1 < w && grid[i][j + 1] && m & 2 == 0 {
            v = std::cmp::max(v, 1 + maximum_matching(cache, grid, ni, nj, (m | 2) >> 1));
        }
        if i + 1 < h {
            for mj in if j > 0 { j - 1 } else { 0 }..1 + if j < w - 1 { j + 1 } else { w - 1 } {
                let mb = 1 << (w + mj - j);
                if grid[i + 1][mj] && m & mb == 0 {
                    v = std::cmp::max(v, 1 + maximum_matching(cache, grid, ni, nj, (m | mb) >> 1));
                }
            }
        }
    }
    cache[i][j][m] = Some(v);
    v
}
