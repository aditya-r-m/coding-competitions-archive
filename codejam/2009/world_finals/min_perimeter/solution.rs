lib::run!();

fn read() -> Vec<[usize; 2]> {
    let mut points: Vec<[usize; 2]> = Vec::new();
    lib::input!(usize as n);
    for _ in 0..n {
        lib::input!([usize; 2] as point);
        points.push(point);
    }
    points
}

fn solve(points: Vec<[usize; 2]>) -> String {
    format!("TODO")
}
