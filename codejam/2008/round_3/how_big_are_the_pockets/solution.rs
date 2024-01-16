use std::collections::HashMap;

lib::run!();

fn read() -> Vec<char> {
    lib::input!(usize as mut l);
    let mut walk = Vec::new();
    while l > 0 {
        lib::input!(Vec<String> as line);
        l -= line.len() >> 1;
        for i in (0..line.len()).step_by(2) {
            for _ in 0..line[i + 1].parse::<usize>().unwrap() {
                walk.append(&mut line[i].chars().collect::<Vec<char>>());
            }
        }
    }
    walk
}

fn solve(walk_chars: Vec<char>) -> String {
    let apply = |c: char, (x, y, dx, dy): (i64, i64, i64, i64)| -> (i64, i64, i64, i64) {
        match c {
            'F' => (x + dx, y + dy, dx, dy),
            'L' => (x, y, -dy, dx),
            'R' => (x, y, dy, -dx),
            _ => unreachable!(),
        }
    };
    let mut horizontal_edges: Vec<(i64, i64)> = Vec::new();
    let mut vertical_edges: Vec<(i64, i64)> = Vec::new();
    let mut position = (0i64, 0i64, 1i64, 0i64);
    for &step in walk_chars.iter() {
        position = apply(step, position);
        if step == 'F' {
            let (x, y, dx, dy) = position;
            if dx != 0 {
                horizontal_edges.push((x - std::cmp::max(0, dx), y));
            } else {
                vertical_edges.push((x, y - std::cmp::max(0, dy)));
            }
        }
    }
    horizontal_edges.sort();
    let mut covered_area = 0i64;
    for h in (0..horizontal_edges.len()).step_by(2) {
        covered_area += horizontal_edges[h + 1].1 - horizontal_edges[h].1;
    }
    let mut total_area = 0i64;
    for h in 0..horizontal_edges.len() - 1 {
        if horizontal_edges[h].0 != horizontal_edges[h + 1].0 {
            continue;
        }
        let x = horizontal_edges[h].0;
        for y in horizontal_edges[h].1..horizontal_edges[h + 1].1 {
            vertical_edges.push((x, y));
            vertical_edges.push((x + 1, y));
        }
    }
    let mut min_yx: HashMap<i64, i64> = HashMap::new();
    let mut max_yx: HashMap<i64, i64> = HashMap::new();
    for (x, y) in vertical_edges.into_iter() {
        min_yx.insert(y, std::cmp::min(x, *min_yx.get(&y).unwrap_or(&x)));
        max_yx.insert(y, std::cmp::max(x, *max_yx.get(&y).unwrap_or(&x)));
    }
    for y in min_yx.keys() {
        total_area += max_yx[&y] - min_yx[&y];
    }
    format!("{}", total_area - covered_area)
}
