#[derive(Clone, Copy)]
struct Fraction {
    i: u128,
    n: u128,
    d: u128,
}

impl Fraction {
    fn new(i: u128, n: u128, d: u128) -> Fraction {
        Fraction { i, n, d }.normalize()
    }
    fn normalize(mut self) -> Fraction {
        let mut g = self.d;
        let mut h = self.n;
        while h % g > 0 {
            (h, g) = (g, h % g);
        }
        self.n /= g;
        self.d /= g;
        self.i += self.n / self.d;
        self.n %= self.d;
        self
    }
}

impl std::ops::Add<Fraction> for Fraction {
    type Output = Fraction;
    fn add(self, rhs: Fraction) -> Fraction {
        Fraction::new(
            self.i + rhs.i,
            self.n * rhs.d + self.d * rhs.n,
            self.d * rhs.d,
        )
    }
}

impl std::ops::Mul<Fraction> for Fraction {
    type Output = Fraction;
    fn mul(self, rhs: Fraction) -> Fraction {
        Fraction::new(self.i * rhs.i, 0, 1)
            + Fraction::new(0, self.i * rhs.n, rhs.d)
            + Fraction::new(0, rhs.i * self.n, self.d)
            + Fraction::new(0, self.n * rhs.n, self.d * rhs.d)
    }
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}/{}", self.i, self.n, self.d)
    }
}

lib::run!();

fn read() -> (u128, Vec<Vec<u128>>) {
    lib::input!([u128; 2] as [n, t]);
    let mut schedules: Vec<Vec<u128>> = Vec::new();
    for _ in 0..t {
        lib::input!(Vec<u128> as schedule);
        schedules.push(schedule);
        schedules.last_mut().unwrap()[0] = 1;
    }
    (n, schedules)
}

fn solve((n, schedules): (u128, Vec<Vec<u128>>)) -> String {
    let mut result = Fraction::new(0, 0, 1);
    let mut coverage: Vec<usize> = vec![0; schedules.len()];
    for i in 1..=n {
        let mut done = true;
        for (j, schedule) in schedules.iter().enumerate() {
            if coverage[j] < schedule.len() {
                done = false;
                if schedule[coverage[j]] <= i {
                    coverage[j] += 1;
                }
            }
        }
        let mut current_result = Fraction::new(0, 0, 1);
        for (vi, &v) in coverage.iter().enumerate() {
            current_result = current_result + Fraction::new(0, v as u128, n);
            for &u in coverage.iter().skip(vi + 1) {
                current_result = current_result + Fraction::new(0, (2 * v * u) as u128, n * n);
            }
        }
        result = result + current_result;
        if done {
            result = result + current_result * Fraction::new(n - i, 0, 1);
            break;
        }
    }
    format!("{}", result)
}
