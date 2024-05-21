lib::run!();

fn read() -> Vec<Vec<usize>> {
    lib::input!([usize; 2] as [h, _]);
    let mut grid: Vec<Vec<usize>> = Vec::new();
    for _ in 0..h {
        lib::input!(Vec<usize> as row);
        grid.push(row);
    }
    grid
}

fn solve(grid: Vec<Vec<usize>>) -> String {
    let h = grid.len();
    let w = grid[0].len();
    let flat_grid = grid.into_iter().flat_map(|row| row).collect::<Vec<usize>>();
    let mut disjoint_set = lib::collections::DisjointSet::new(h * w);
    for i in 0..h * w {
        let [mut v, mut j] = [flat_grid[i], i];
        if i >= w && flat_grid[i - w] < v {
            [v, j] = [flat_grid[i - w], i - w];
        }
        if i % w > 0 && flat_grid[i - 1] < v {
            [v, j] = [flat_grid[i - 1], i - 1];
        }
        if i % w < w - 1 && flat_grid[i + 1] < v {
            [v, j] = [flat_grid[i + 1], i + 1];
        }
        if i + w < h * w && flat_grid[i + w] < v {
            j = i + w;
        }
        disjoint_set.union(i, j);
    }
    let mut char_map: std::collections::HashMap<usize, char> = std::collections::HashMap::new();
    let mut char_cur = '`';
    let mut output = String::from("\u{8}");
    for i in 0..h * w {
        output.push(if i % w == 0 { '\n' } else { ' ' });
        output.push(
            *char_map.entry(disjoint_set.get_root(i)).or_insert_with(|| {
                char_cur = char::from_u32(char_cur as u32 + 1).unwrap();
                char_cur
            }),
        );
    }
    output
}
