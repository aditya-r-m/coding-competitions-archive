lib::run!();

struct TestCase {
    origin: [i64; 2],
    limits: [i64; 2],
    vectors: [[i64; 2]; 2],
}

fn read() -> TestCase {
    lib::input!([i64; 2] as limits);
    lib::input!([i64; 2] as b0);
    lib::input!([i64; 2] as b1);
    lib::input!([i64; 2] as origin);
    TestCase {
        origin,
        limits,
        vectors: [b0, b1],
    }
}

fn is_contained(limits: [i64; 2], position: [i64; 2]) -> bool {
    position[0] >= 0 && position[1] >= 0 && position[0] < limits[0] && position[1] < limits[1]
}

fn are_collinear(mut vectors: [[i64; 2]; 2]) -> bool {
    for d in 2..1 + std::cmp::max(
        vectors[0][0].abs(),
        std::cmp::max(
            vectors[0][1].abs(),
            std::cmp::max(vectors[1][0].abs(), vectors[1][1].abs()),
        ),
    ) {
        if vectors[0][0] % d == 0 && vectors[0][1] % d == 0 {
            vectors[0][0] /= d;
            vectors[0][1] /= d;
        }
        if vectors[1][0] % d == 0 && vectors[1][1] % d == 0 {
            vectors[1][0] /= d;
            vectors[1][1] /= d;
        }
    }
    vectors[0] == vectors[1] || (vectors[0][0] == -vectors[1][0] && vectors[0][1] == -vectors[1][1])
}

fn breadth_first_search(
    TestCase {
        origin,
        limits,
        vectors,
    }: TestCase,
) -> usize {
    let mut visited: std::collections::HashSet<[i64; 2]> = std::collections::HashSet::new();
    let mut queue: std::collections::VecDeque<[i64; 2]> = std::collections::VecDeque::new();
    queue.push_back(origin);
    while let Some(point) = queue.pop_front() {
        if visited.contains(&point) || !is_contained(limits, point) {
            continue;
        }
        visited.insert(point);
        for ball in vectors {
            queue.push_back([point[0] + ball[0], point[1] + ball[1]]);
        }
    }
    visited.len()
}

fn perimeter_search(
    TestCase {
        origin,
        limits,
        vectors,
    }: TestCase,
) -> i64 {
    let mut c = 0i64;
    let mut u = 0i64;
    let mut v0 = 0i64;
    let mut v1 = 0i64;
    let get_point = |u: i64, v: i64| {
        [
            origin[0] + u * vectors[0][0] + v * vectors[1][0],
            origin[1] + u * vectors[0][1] + v * vectors[1][1],
        ]
    };
    loop {
        while !is_contained(limits, get_point(u, v0))
            || !is_contained(limits, get_point(std::cmp::max(0, u - 1), v0))
        {
            v0 += 1;
            if get_point(u, v0)[0] < 0 && vectors[1][0] <= 0
                || get_point(u, v0)[0] >= limits[0] && vectors[1][0] >= 0
                || get_point(u, v0)[1] < 0 && vectors[1][1] <= 0
                || get_point(u, v0)[1] >= limits[1] && vectors[1][1] >= 0
            {
                return c;
            }
        }
        v1 = std::cmp::max(v0, v1);
        if is_contained(limits, get_point(u, v1)) {
            while is_contained(limits, get_point(u, v1 + 1)) {
                v1 += 1;
            }
        } else {
            while !is_contained(limits, get_point(u, v1)) {
                v1 -= 1;
            }
        }
        c += 1 + v1 - v0;
        u += 1;
    }
}

fn solve(test_case: TestCase) -> String {
    if are_collinear(test_case.vectors) {
        format!("{}", breadth_first_search(test_case))
    } else {
        format!("{}", perimeter_search(test_case))
    }
}
