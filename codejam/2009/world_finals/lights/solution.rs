const E: f64 = 1e-10f64;

lib::run!();

fn read() -> ([[f64; 2]; 2], Vec<[f64; 3]>) {
    lib::input!([f64; 2] as source_r);
    lib::input!([f64; 2] as source_g);
    lib::input!(usize as n);
    let mut circles: Vec<[f64; 3]> = Vec::new();
    for _ in 0..n {
        lib::input!([f64; 3] as circle);
        circles.push(circle);
    }
    ([source_r, source_g], circles)
}

fn solve((sources, circles): ([[f64; 2]; 2], Vec<[f64; 3]>)) -> String {
    let mut area_b = 0f64;
    let mut area_r = 0f64;
    let mut area_g = 0f64;
    let mut area_y = 0f64;

    let mut d = 1f64;
    let mut grid_points: Vec<[f64; 2]> = Vec::new();
    let mut y = 0f64;
    while y + E < 100f64 {
        let mut x = 0f64;
        while x + E < 100f64 {
            grid_points.push([x, y]);
            x += d;
        }
        y += d;
    }
    let mut next_grid_points: Vec<[f64; 2]> = Vec::new();
    for _ in 0..8 {
        while let Some(point) = grid_points.pop() {
            let corners = [
                [point[0], point[1]],
                [point[0], point[1] + d],
                [point[0] + d, point[1]],
                [point[0] + d, point[1] + d],
            ];
            let next_points = [
                [point[0], point[1]],
                [point[0], point[1] + d / 2f64],
                [point[0] + d / 2f64, point[1]],
                [point[0] + d / 2f64, point[1] + d / 2f64],
            ];
            let mut circle_containment = false;
            for circle in circles.iter() {
                let circle_containments = corners.map(|corner| is_contained(&corner, circle));
                if circle_containments.iter().any(|&v| v) {
                    circle_containment = true;
                    if !circle_containments.iter().all(|&v| v) {
                        for next_point in next_points {
                            next_grid_points.push(next_point);
                        }
                    }
                    break;
                }
            }
            if circle_containment {
                continue;
            }
            let mut dark = [false; 2];
            let mut partial_dark = false;
            for (i, source) in sources.iter().enumerate() {
                for circle in circles.iter() {
                    let darks = corners.map(|corner| is_intercepted(source, &corner, circle));
                    if darks.iter().all(|&v| v) {
                        dark[i] = true;
                        break;
                    }
                    if darks.iter().any(|&v| v) {
                        partial_dark = true;
                        for next_point in next_points {
                            next_grid_points.push(next_point);
                        }
                        break;
                    }
                }
                if partial_dark {
                    break;
                }
            }
            if partial_dark {
                continue;
            }
            match dark {
                [true, true] => area_b += d * d,
                [false, true] => area_r += d * d,
                [true, false] => area_g += d * d,
                [false, false] => area_y += d * d,
            }
        }
        (grid_points, next_grid_points) = (next_grid_points, grid_points);
        d /= 2f64;
    }
    format!(
        "\n{:10.15e}\n{:10.15e}\n{:10.15e}\n{:10.15e}",
        area_b, area_r, area_g, area_y
    )
}

fn is_contained(point: &[f64; 2], circle: &[f64; 3]) -> bool {
    (circle[0] - point[0]).powi(2) + (circle[1] - point[1]).powi(2) - E < circle[2].powi(2)
}

fn is_intercepted(source: &[f64; 2], point: &[f64; 2], circle: &[f64; 3]) -> bool {
    let dx = point[0] - source[0];
    let dy = point[1] - source[1];
    if dx.abs() < dy.abs() {
        return is_intercepted(
            &[source[1], source[0]],
            &[point[1], point[0]],
            &[circle[1], circle[0], circle[2]],
        );
    }
    let m = dy / dx;
    // (y - sy) = m * (x - sx)
    // y = m * x - m * sx + sy = m * x + o
    let o0 = -m * source[0] + source[1];
    // cr^2 = (y - cy)^2 + (x - cx)^2
    // cr^2 = (mx + o0 - cy)^2  + (x - cx)^2 = (mx + o1)^2 + (x - cx)^2
    let o1 = o0 - circle[1];
    // 0 = (m^2 + 1) * x^2 + 2 * (m * o1 - cx) * x + (o1^2 + cx^2 - cr^2) = a * x^2 + b * x + c
    let a = m.powi(2) + 1f64;
    let b = 2f64 * (m * o1 - circle[0]);
    let c = o1.powi(2) + circle[0].powi(2) - circle[2].powi(2);
    if b.powi(2) < 4f64 * a * c {
        return false;
    }
    let x = (1f64 / (2f64 * a)) * (-b + (b.powi(2) - 4f64 * a * c).sqrt());
    !((x + E < source[0] && x + E < point[0]) || (x > E + source[0] && x > E + point[0]))
}
