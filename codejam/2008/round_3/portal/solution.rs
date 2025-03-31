use std::collections::HashSet;

lib::run!();

type Point = (usize, usize);
struct TestCase {
    grid: Vec<Vec<bool>>,
    source: Point,
    target: Point,
}

fn read() -> TestCase {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut source: Point = (0, 0);
    let mut target: Point = (0, 0);

    lib::input!([usize; 2] as [row_count, _]);
    for r in 1..1 + row_count {
        lib::input!(buffer);
        let mut row = buffer.chars().collect::<Vec<char>>();

        row.push('#');
        row.insert(0, '#');
        grid.push(row.iter().map(|&c| c != '#').collect::<Vec<bool>>());
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'O' {
                source = (r, c);
            } else if ch == 'X' {
                target = (r, c);
            }
        }
    }
    grid.push(vec![false; grid[0].len()]);
    grid.insert(0, vec![false; grid[0].len()]);

    TestCase {
        grid,
        source,
        target,
    }
}

fn solve(
    TestCase {
        grid,
        source,
        target,
    }: TestCase,
) -> String {
    if let Some(shorted_path) = bfs(source, target, grid) {
        format!("{shorted_path}")
    } else {
        "THE CAKE IS A LIE".to_string()
    }
}

fn bfs(source: Point, target: Point, grid: Vec<Vec<bool>>) -> Option<usize> {
    let mut distance = 0usize;
    let mut next_layer: Vec<(Point, Point)> = Vec::new();
    let mut current_layer: Vec<(Point, Point)> = Vec::new();
    let mut visited_state_set: HashSet<(Point, Point)> = HashSet::new();

    current_layer.push((source, source));
    visited_state_set.insert((source, source));

    while !current_layer.is_empty() {
        while let Some((location, portal)) = current_layer.pop() {
            if location == target {
                return Some(distance);
            }
            for (dx, dy) in [
                (0isize, -1isize),
                (0isize, 1isize),
                (-1isize, 0isize),
                (1isize, 0isize),
            ] {
                let next_location = (
                    location.0.wrapping_add_signed(dx),
                    location.1.wrapping_add_signed(dy),
                );
                if grid[next_location.0][next_location.1]
                    && !visited_state_set.contains(&(next_location, portal))
                {
                    visited_state_set.insert((next_location, portal));
                    next_layer.push((next_location, portal));
                }
                if !grid[next_location.0][next_location.1]
                    && !visited_state_set.contains(&(portal, portal))
                {
                    visited_state_set.insert((portal, portal));
                    next_layer.push((portal, portal));
                }

                let mut next_portal = location;
                let mut candidate_portal = location;
                while {
                    candidate_portal = (
                        candidate_portal.0.wrapping_add_signed(dx),
                        candidate_portal.1.wrapping_add_signed(dy),
                    );
                    grid[candidate_portal.0][candidate_portal.1]
                } {
                    next_portal = candidate_portal
                }
                if !visited_state_set.contains(&(location, next_portal)) {
                    visited_state_set.insert((location, next_portal));
                    current_layer.push((location, next_portal));
                }
            }
        }
        (current_layer, next_layer) = (next_layer, current_layer);
        distance += 1;
    }
    None
}
