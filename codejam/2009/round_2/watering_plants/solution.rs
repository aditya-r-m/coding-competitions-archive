lib::run!();

fn read() -> Vec<[f64; 3]> {
    lib::input!(usize as n);
    let mut circles: Vec<[f64; 3]> = Vec::with_capacity(n);
    for _ in 0..n {
        lib::input!([f64; 3] as circle);
        circles.push(circle);
    }
    circles
}

fn full_coverage(circles: &Vec<[f64; 3]>, r: f64) -> bool {
    let mut circle_covers: Vec<[f64; 2]> = Vec::new();
    for &[x, y, _] in circles.iter() {
        circle_covers.push([x, y]);
    }
    for (i, &[mut x0, mut y0, r0]) in circles.iter().enumerate() {
        for &[mut x1, mut y1, r1] in circles.iter().skip(i + 1) {
            let mut transposed = false;
            if (y1 - y0).abs() < 1e-8 {
                [x0, x1, y0, y1] = [y0, y1, x0, x1];
                transposed = true;
            }
            // (x, y) lie at intesection of the circles (x - xi)^2 + (y - yi)^2 = (r - ri)^2 = dri^2
            // x^2 - 2*xi*x + xi^2 + y^2 - 2*yi*y + yi^2 = dri^2
            // x^2 + y^2 - 2*xi*x - 2*yi*y = dri^2 - xi^2 - yi^2 = ci
            let c0 = (r - r0).powf(2f64) - x0 * x0 - y0 * y0;
            let c1 = (r - r1).powf(2f64) - x1 * x1 - y1 * y1;
            // subtracting eqn (1) from (0),
            // 2*(x1 - x0)*x + 2*(y1 - y0)*y = c0 - c1
            // y = (c0 - c1)/(2*(y1 - y0)) - (x1 - x0)/(y1 - y0)*x = c + m*x
            let c = (c0 - c1) / (2f64 * (y1 - y0));
            let m = (x0 - x1) / (y1 - y0);
            // substituting 'y' in eqn (0),
            // (m^2 + 1)*x^2 + 2*m*x*c + c^2 - 2*x0*x - 2*y0*(m*x + c) = c0
            // (m^2 + 1)*x^2 + (2*m*c - 2*x0 - 2*y0*m)*x + c^2 - 2*y0*c - c0 = qa*x^2 + qb^x + qc = 0
            let qa = m * m + 1f64;
            let qb = 2f64 * m * c - 2f64 * x0 - 2f64 * y0 * m;
            let qc = c * c - 2f64 * y0 * c - c0;
            let qd = qb * qb - 4f64 * qa * qc;
            if qd < 0f64 {
                continue;
            }
            let mut cx0 = (-qb + qd.sqrt()) / (2f64 * qa);
            let mut cx1 = (-qb - qd.sqrt()) / (2f64 * qa);
            let mut cy0 = m * cx0 + c;
            let mut cy1 = m * cx1 + c;
            if transposed {
                [cx0, cx1, cy0, cy1] = [cy0, cy1, cx0, cx1];
            }
            circle_covers.push([cx0, cy0]);
            circle_covers.push([cx1, cy1]);
        }
    }
    for &[x0, y0] in circle_covers.iter() {
        for &[x1, y1] in circle_covers.iter() {
            if circles.iter().all(|&[ix, iy, ir]| {
                ((ix - x0).powf(2f64) + (iy - y0).powf(2f64))
                    .min((ix - x1).powf(2f64) + (iy - y1).powf(2f64))
                    < (r - ir).powf(2f64) + 1e-8
                    && r > ir
            }) {
                return true;
            }
        }
    }
    false
}

fn solve(circles: Vec<[f64; 3]>) -> String {
    let mut r_min = 0f64;
    let mut r_max = 1f64;
    while !full_coverage(&circles, r_max) {
        r_max *= 2f64;
    }
    while r_max - r_min > 1e-10 {
        let r_mid = (r_min + r_max) / 2f64;
        if !full_coverage(&circles, r_mid) {
            r_min = r_mid;
        } else {
            r_max = r_mid;
        }
    }
    format!("{:.9}", r_max)
}
