use std::collections::HashSet;

lib::run!();

struct TestCase {
    engine_count: usize,
    queries: Vec<String>,
}

fn read() -> TestCase {
    lib::input!(usize as engine_count);
    for _ in 0..engine_count {
        lib::input!(_e);
    }
    lib::input!(usize as q);
    let mut queries: Vec<String> = Vec::new();
    for _ in 0..q {
        lib::input!(e);
        queries.push(e);
    }
    TestCase {
        engine_count,
        queries,
    }
}

fn solve(
    TestCase {
        engine_count,
        queries,
    }: TestCase,
) -> String {
    let mut excluded_engines: HashSet<String> = HashSet::new();
    let mut minimum_reassignment_count: usize = 0;
    for query in queries.into_iter() {
        if excluded_engines.len() == engine_count - 1 && !excluded_engines.contains(&query) {
            minimum_reassignment_count += 1;
            excluded_engines.clear();
        }
        excluded_engines.insert(query);
    }
    format!("{}", minimum_reassignment_count)
}
