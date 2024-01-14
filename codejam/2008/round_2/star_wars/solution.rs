lib::run!();

struct Ship {
    x: f64,
    y: f64,
    z: f64,
    p: f64,
}

fn read() -> Vec<Ship> {
    lib::input!(usize as n);
    let mut ships: Vec<Ship> = Vec::with_capacity(n);
    for _ in 0..n {
        lib::input!([f64; 4] as [x, y, z, p]);
        ships.push(Ship { x, y, z, p });
    }
    ships
}

fn cc(ships: &[Ship], i: u32, t: f64) -> f64 {
    ships
        .iter()
        .map(|ship| {
            ship.x
                + (-1i64).pow(i >> 2) as f64 * ship.y
                + (-1i64).pow(i >> 1) as f64 * ship.z
                + (-1i64).pow(i) as f64 * t * ship.p
        })
        .fold(f64::NAN, if i & 1 == 0 { f64::min } else { f64::max })
}

fn reachable(ships: &[Ship], t: f64) -> bool {
    let c = |i| cc(ships, i, t);
    f64::min(c(0) + c(6), c(2) + c(4)) >= f64::max(c(1) + c(7), c(3) + c(5))
        && c(0) >= c(1)
        && c(2) >= c(3)
        && c(4) >= c(5)
        && c(6) >= c(7)
}

fn solve(ships: Vec<Ship>) -> String {
    let mut upper_bound = 1f64;
    while !reachable(&ships, upper_bound) {
        upper_bound *= 2f64;
    }
    let mut lower_bound = 0f64;
    for _ in 0..128 {
        let mid_point = (upper_bound + lower_bound) / 2f64;
        if reachable(&ships, mid_point) {
            upper_bound = mid_point;
        } else {
            lower_bound = mid_point;
        }
    }

    format!("{upper_bound}")
}
