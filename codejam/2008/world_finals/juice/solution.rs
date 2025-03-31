const L: usize = 10_000;

lib::run!();

struct TestCase {
    preferences: Vec<[usize; 3]>,
}

fn read() -> TestCase {
    let mut preferences: Vec<[usize; 3]> = Vec::new();
    lib::input!(usize as n);
    for _ in 0..n {
        lib::input!([usize; 3] as preference);
        preferences.push(preference);
    }
    TestCase { preferences }
}

struct ProjectedPreference {
    a: usize,
    b: usize,
    l: usize,
}

impl ProjectedPreference {
    fn new([a, b, c]: [usize; 3]) -> ProjectedPreference {
        ProjectedPreference {
            a,
            b,
            l: L - (a + b + c),
        }
    }

    fn corners(&self) -> Vec<[usize; 2]> {
        vec![
            [self.a, self.b],
            [self.a + self.l, self.b],
            [self.a, self.b + self.l],
        ]
    }

    fn contains(&self, [x, y]: [usize; 2]) -> bool {
        x >= self.a && y >= self.b && (x + y) <= self.a + self.b + self.l
    }

    fn intersection(&self, other: &ProjectedPreference) -> Option<ProjectedPreference> {
        if !self.corners().into_iter().any(|c| other.contains(c))
            && !other.corners().into_iter().any(|c| self.contains(c))
        {
            return None;
        }
        let a = std::cmp::max(self.a, other.a);
        let b = std::cmp::max(self.b, other.b);
        let l = [
            self.a + self.l - a,
            self.b + self.l - b,
            other.a + other.l - a,
            other.b + other.l - b,
            std::cmp::min(
                self.a + 2 * self.l + self.b,
                other.a + 2 * other.l + other.b,
            ) - std::cmp::max(self.a + self.b, other.a + other.b),
        ]
        .into_iter()
        .fold(usize::MAX, std::cmp::min);
        Some(ProjectedPreference { a, b, l })
    }
}

fn solve(TestCase { preferences }: TestCase) -> String {
    let projected_preferences = preferences
        .into_iter()
        .map(ProjectedPreference::new)
        .collect::<Vec<ProjectedPreference>>();
    let mut projected_preference_intersections: Vec<[usize; 2]> = projected_preferences
        .iter()
        .flat_map(|p| p.corners())
        .collect::<Vec<[usize; 2]>>();
    for p in projected_preferences.iter() {
        for q in projected_preferences.iter() {
            if let Some(r) = p.intersection(q) {
                projected_preference_intersections.append(&mut r.corners());
            }
        }
    }
    projected_preference_intersections.sort();
    projected_preference_intersections.dedup();
    format!(
        "{}",
        projected_preference_intersections
            .into_iter()
            .map(|i| projected_preferences
                .iter()
                .filter(|q| q.contains(i))
                .count())
            .fold(0, std::cmp::max)
    )
}
