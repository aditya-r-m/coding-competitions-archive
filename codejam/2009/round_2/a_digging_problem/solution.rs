use std::collections::{BinaryHeap, HashSet};

lib::run!();

fn read() -> (Vec<Vec<bool>>, usize) {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    lib::input!([usize; 3] as [r, _, total_health_points]);
    for _ in 0..r {
        lib::input!(Vec<char> as row);
        grid.push(row.into_iter().map(|c| c == '#').collect::<Vec<bool>>());
    }
    (grid, total_health_points)
}

fn solve((mut grid, total_health_points): (Vec<Vec<bool>>, usize)) -> String {
    for row in grid.iter_mut() {
        row.push(true);
        row.insert(0, true);
    }
    let mut queue: BinaryHeap<(isize, (usize, usize), (isize, usize))> = BinaryHeap::new();
    let mut visited: HashSet<((usize, usize), isize)> = HashSet::new();
    queue.push((0, (0, 1), (0, total_health_points)));
    while let Some((score, (r, c), (dug_range, health_points))) = queue.pop() {
        if r + 1 == grid.len() {
            return format!("Yes {}", -score);
        }
        if !grid[r + 1][c] {
            if health_points > 0 {
                queue.push((score, (r + 1, c), (0, health_points - 1)));
            }
            continue;
        }
        if visited.contains(&((r, c), dug_range)) {
            continue;
        }
        visited.insert(((r, c), dug_range));
        let mut platform_start = c;
        while grid[r + 1][platform_start - 1]
            && (dug_range < platform_start as isize - c as isize || !grid[r][platform_start - 1])
        {
            platform_start -= 1;
        }
        if dug_range < platform_start as isize - c as isize || !grid[r][platform_start - 1] {
            queue.push((
                score,
                (r + 1, platform_start - 1),
                (0, total_health_points - 1),
            ));
        }
        let mut platform_end = c;
        while grid[r + 1][platform_end + 1]
            && (dug_range > platform_end as isize - c as isize || !grid[r][platform_end + 1])
        {
            platform_end += 1;
        }
        if dug_range > platform_end as isize - c as isize || !grid[r][platform_end + 1] {
            queue.push((
                score,
                (r + 1, platform_end + 1),
                (0, total_health_points - 1),
            ));
        }
        for nxt_c in platform_start..platform_end {
            for nxt_dug_range in platform_start as isize - nxt_c as isize..=0isize {
                queue.push((
                    score + nxt_dug_range - 1isize,
                    (r + 1, nxt_c),
                    (nxt_dug_range, total_health_points - 1),
                ));
            }
        }
        for nxt_c in platform_start + 1..=platform_end {
            for nxt_dug_range in 0isize..=platform_end as isize - nxt_c as isize {
                queue.push((
                    score - nxt_dug_range - 1isize,
                    (r + 1, nxt_c),
                    (nxt_dug_range, total_health_points - 1),
                ));
            }
        }
    }
    format!("No")
}
