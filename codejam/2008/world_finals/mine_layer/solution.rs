lib::run!();

fn read() -> Vec<Vec<usize>> {
    let mut grid: Vec<Vec<usize>> = Vec::new();
    lib::input!([usize; 2] as [r, _]);
    for _ in 0..r {
        lib::input!(Vec<usize> as row);
        grid.push(row);
    }
    grid
}

trait CountMines {
    fn count_mines(&self) -> usize;
}

impl CountMines for usize {
    fn count_mines(&self) -> usize {
        *self
    }
}

impl<T> CountMines for Vec<T>
where
    T: CountMines,
{
    fn count_mines(&self) -> usize {
        self.iter()
            .enumerate()
            .filter(|&(i, _)| i % 3 == 1 - ((self.len() % 3) & 1))
            .map(|(_, v)| v.count_mines())
            .sum::<usize>()
    }
}

fn solve(grid: Vec<Vec<usize>>) -> String {
    let mine_count_all = grid.count_mines();
    let mine_count_top = grid
        .clone()
        .into_iter()
        .take(grid.len() >> 1)
        .collect::<Vec<Vec<usize>>>()
        .count_mines();
    let mine_count_bottom = grid
        .clone()
        .into_iter()
        .rev()
        .take(grid.len() >> 1)
        .collect::<Vec<Vec<usize>>>()
        .count_mines();
    format!(
        "{}",
        std::cmp::max(mine_count_all, mine_count_top + mine_count_bottom)
            - std::cmp::min(mine_count_all, mine_count_top + mine_count_bottom)
    )
}
