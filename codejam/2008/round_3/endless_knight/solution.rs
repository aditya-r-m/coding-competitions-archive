use lib::collections::ModInt;

lib::run!();

const P: usize = 10007;

fn mod_int(i: usize) -> ModInt {
    ModInt::new(i, P)
}

struct TestCase {
    h: usize,
    w: usize,
    rocks: Vec<(usize, usize)>,
}

fn read() -> TestCase {
    lib::input!([usize; 3] as [h, w, r]);
    let mut rocks: Vec<(usize, usize)> = Vec::with_capacity(r);
    for _ in 0..r {
        lib::input!([usize; 2] as [rock_r, rock_c]);
        rocks.push((rock_r, rock_c));
    }
    TestCase { h, w, rocks }
}

fn solve(TestCase { h, w, mut rocks }: TestCase) -> String {
    let mut factorials = [mod_int(1); P];
    for i in 1..P {
        factorials[i] = mod_int(i) * factorials[i - 1];
    }
    let c = |mut n: usize, mut r: usize| -> ModInt {
        let mut result = mod_int(1);
        while n > 0 || r > 0 {
            if r % P > n % P {
                return mod_int(0);
            }
            result *=
                factorials[n % P] / (factorials[r % P] * factorials[(P + (n % P) - (r % P)) % P]);
            n /= P;
            r /= P;
        }
        result
    };
    let count_knight_paths = |x: usize, y: usize| -> ModInt {
        if y > 2 * x {
            return mod_int(0);
        }
        let i = 2 * x - y;
        if i % 3 != 0 {
            return mod_int(0);
        }
        let r = i / 3;
        let n = x - r;
        if n < r {
            return mod_int(0);
        }
        c(n, r)
    };
    rocks.push((h, w));
    rocks.sort_by_key(|(x, y)| x + y);
    let mut solution = mod_int(0);
    for mut set in 0..(1 << (rocks.len() - 1)) {
        set |= 1 << (rocks.len() - 1);
        let mut pre = (1, 1);
        let mut sol = -mod_int(1);
        for (i, &(x, y)) in rocks.iter().enumerate() {
            if set & (1 << i) == 0 {
                continue;
            }
            if x < pre.0 || y < pre.1 {
                sol = mod_int(0);
                continue;
            }
            sol *= -count_knight_paths(x - pre.0, y - pre.1);
            pre = (x, y);
        }
        solution += sol;
    }
    format!("{}", solution.i)
}
