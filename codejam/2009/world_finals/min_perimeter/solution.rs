use std::collections::{BTreeSet, VecDeque};

lib::run!();

fn read() -> Vec<[f64; 2]> {
    let mut points: Vec<[f64; 2]> = Vec::new();
    lib::input!(usize as n);
    for _ in 0..n {
        lib::input!([f64; 2] as point);
        points.push(point);
    }
    points
}

fn solve(mut points: Vec<[f64; 2]>) -> String {
    points.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mask: BTreeSet<usize> = BTreeSet::from_iter(0..points.len());
    format!("{:.15}", min_perimeter(&points, mask))
}

fn distance(p0: [f64; 2], p1: [f64; 2]) -> f64 {
    ((p0[0] - p1[0]).powi(2) + (p0[1] - p1[1]).powi(2)).sqrt()
}

fn perimeter(p0: [f64; 2], p1: [f64; 2], p2: [f64; 2]) -> f64 {
    distance(p0, p1) + distance(p1, p2) + distance(p2, p0)
}

fn min_perimeter(points: &Vec<[f64; 2]>, mask: BTreeSet<usize>) -> f64 {
    if mask.len() < 3 {
        return f64::INFINITY;
    }
    let ys = mask.iter().map(|&m| points[m][1]).collect::<Vec<f64>>();
    let mid_y = ys[ys.len() >> 1];
    let mask_left = mask
        .iter()
        .cloned()
        .filter(|&m| points[m][1] < mid_y)
        .collect::<BTreeSet<usize>>();
    let mask_right = mask
        .iter()
        .cloned()
        .filter(|&m| points[m][1] > mid_y)
        .collect::<BTreeSet<usize>>();
    let mut result = min_perimeter(points, mask_left).min(min_perimeter(points, mask_right));
    let mut window: VecDeque<[f64; 2]> = VecDeque::new();
    for m in mask.into_iter() {
        if points[m][1] > mid_y - result && points[m][1] < mid_y + result {
            window.push_front(points[m]);
            if window.len() > 16 {
                window.pop_back();
            }
            for (i0, &p0) in window.iter().enumerate() {
                for (i1, &p1) in window.iter().enumerate().skip(i0 + 1) {
                    for &p2 in window.iter().skip(i1 + 1) {
                        result = result.min(perimeter(p0, p1, p2));
                    }
                }
            }
        }
    }
    result
}
