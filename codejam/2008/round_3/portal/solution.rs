use std::collections::HashSet;
use std::io::stdin;

type Point = (usize, usize);

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

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let test_case_count = buffer.trim().parse::<usize>().unwrap();
    buffer.clear();

    for test_case_index in 1..1 + test_case_count {
        stdin().read_line(&mut buffer).unwrap();
        let row_count = buffer.split(" ").next().unwrap().parse::<usize>().unwrap();
        buffer.clear();

        let mut grid: Vec<Vec<bool>> = Vec::new();
        let mut source: Point = (0, 0);
        let mut target: Point = (0, 0);

        for r in 1..1 + row_count {
            stdin().read_line(&mut buffer).unwrap();
            let mut row = buffer.trim().chars().collect::<Vec<char>>();
            buffer.clear();

            row.push('#');
            row.insert(0, '#');
            grid.push(row.iter().map(|&c| c != '#').collect::<Vec<bool>>());
            for c in 0..row.len() {
                if row[c] == 'O' {
                    source = (r, c);
                } else if row[c] == 'X' {
                    target = (r, c);
                }
            }
        }
        grid.push(vec![false; grid[0].len()]);
        grid.insert(0, vec![false; grid[0].len()]);
        println!(
            "Case #{test_case_index}: {}",
            if let Some(shorted_path) = bfs(source, target, grid) {
                format!("{shorted_path}")
            } else {
                "THE CAKE IS A LIE".to_string()
            }
        );
    }
}
