use lib::collections::ModInt;
use std::collections::HashMap;

fn mod_int(v: usize) -> ModInt {
    ModInt {
        i: v as usize,
        m: 10007,
    }
}

lib::run!();

fn read() -> Vec<Vec<usize>> {
    let mut grid: Vec<Vec<usize>> = Vec::new();
    lib::input!([usize; 2] as [r, _]);
    for _ in 0..r {
        lib::input!(Vec<char> as row);
        grid.push(
            row.into_iter()
                .map(|c| {
                    if c == '.' {
                        26
                    } else {
                        c as usize - 'a' as usize
                    }
                })
                .collect::<Vec<usize>>(),
        );
    }
    grid
}

fn solve(grid: Vec<Vec<usize>>) -> String {
    let (n, m) = (grid.len(), grid[0].len());
    let mut path = 0usize;
    for i in 0..n {
        path |= m << (4 * i);
    }
    format!(
        "{}",
        compute(&grid, &mut HashMap::new(), (path, 25, m - 1)).i
    )
}

fn compute(
    grid: &Vec<Vec<usize>>,
    cache: &mut HashMap<(usize, usize, usize), ModInt>,
    (path, c, k): (usize, usize, usize),
) -> ModInt {
    let (n, m) = (grid.len(), grid[0].len());
    let mut result: ModInt;
    if let Some(&v) = cache.get(&(path, c, k)) {
        result = v;
    } else if path == 0 {
        result = mod_int(1);
    } else if c == 0 {
        result = mod_int(1);
        for i in 0..n {
            for j in 0..(path >> (4 * i)) & 15 {
                if k + 1 < j || (grid[i][j] != 26 && grid[i][j] != 0) {
                    result = mod_int(0);
                }
            }
        }
    } else {
        result = compute(
            grid,
            cache,
            if k == 0 {
                (path, c - 1, m - 1)
            } else {
                (path, c, k - 1)
            },
        );
        for i in (0..n).rev() {
            let j = (path >> (4 * i)) & 15;
            if k + 1 < j {
                break;
            }
            if k + 1 == j {
                if grid[i][k] == 26 || grid[i][k] == c {
                    let nxt_path = path - (1 << (4 * i));
                    result += compute(grid, cache, (nxt_path, c, k));
                }
                break;
            }
        }
    }
    cache.insert((path, c, k), result);
    return result;
}
