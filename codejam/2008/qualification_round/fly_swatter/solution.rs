lib::run!();

struct TestCase {
    f: f64,
    l: f64,
    t: f64,
    r: f64,
    g: f64,
}

fn read() -> TestCase {
    lib::input!([f64; 5] as [f, l, t, r, g]);
    TestCase { f, l, t, r, g }
}

fn solve(TestCase { f, l, t, r, g }: TestCase) -> String {
    format!("{:.6}", fly_swatting_probability(f, l, t, r, g))
}

fn fly_swatting_probability(f: f64, mut l: f64, mut t: f64, mut r: f64, mut g: f64) -> f64 {
    let total_area = std::f64::consts::PI * l * l / 4_f64;
    t += f;
    l -= t;
    r += f;
    g -= 2_f64 * f;

    if l <= 0_f64 || g <= 0_f64 {
        return 1_f64;
    }
    let mut clipping_squares: Vec<(f64, f64)> = Vec::new();
    let mut clipping_squares_next: Vec<(f64, f64)> = Vec::new();
    let mut open_area_lower_bound = 0_f64;
    let mut open_area_upper_bound = 0_f64;
    {
        let mut x = r;
        while x < l {
            let mut y = r;
            while x * x + y * y < l * l {
                open_area_upper_bound += g * g;
                if (x + g) * (x + g) + (y + g) * (y + g) < l * l {
                    open_area_lower_bound += g * g;
                } else {
                    clipping_squares.push((x, y));
                }
                y += g + 2_f64 * r;
            }
            x += g + 2_f64 * r;
        }
    }
    if clipping_squares.len() == 0 {
        return 1_f64 - open_area_upper_bound / total_area;
    }

    loop {
        g /= 2_f64;
        for &(x, y) in clipping_squares.iter() {
            for (xs, ys) in [(x, y), (x + g, y), (x, y + g), (x + g, y + g)] {
                if xs * xs + ys * ys < l * l {
                    if (xs + g) * (xs + g) + (ys + g) * (ys + g) < l * l {
                        open_area_lower_bound += g * g;
                    } else {
                        clipping_squares_next.push((xs, ys));
                    }
                } else {
                    open_area_upper_bound -= g * g;
                }
            }

            let solution_lower_bound = 1_f64 - open_area_upper_bound / total_area;
            let solution_upper_bound = 1_f64 - open_area_lower_bound / total_area;
            if solution_upper_bound - solution_lower_bound < 10_f64.powf(-6_f64) {
                return solution_lower_bound;
            }
        }
        (clipping_squares, clipping_squares_next) = (clipping_squares_next, clipping_squares);
        clipping_squares_next.clear();
    }
}
