lib::run!();

type TestCase = Vec<Vec<Option<bool>>>;

fn read() -> TestCase {
    let mut test_case: TestCase = Vec::new();
    lib::input!([usize; 2] as [r, _]);
    for _ in 0..r {
        lib::input!(Vec<char> as row);
        test_case.push(
            row.into_iter()
                .map(|c| match c {
                    '?' => None,
                    '#' => Some(true),
                    '.' => Some(false),
                    _ => panic!("unrecognized input"),
                })
                .collect::<Vec<Option<bool>>>(),
        );
    }
    test_case
}

fn solve(mut test_case: TestCase) -> String {
    for row in test_case.iter_mut() {
        row.push(Some(false));
        row.insert(0, Some(false));
    }
    test_case.push(vec![Some(false); test_case[0].len()]);
    test_case.insert(0, vec![Some(false); test_case[0].len()]);
    for i in 0..test_case.len() {
        for j in (i & 1..test_case[0].len()).step_by(2) {
            if let Some(v) = test_case[i][j] {
                test_case[i][j] = Some(!v);
            }
        }
    }
    let h = test_case.len();
    let w = test_case[0].len();
    let s = h * w;
    let t = s + 1;
    let mut graph: Vec<std::collections::HashMap<usize, usize>> =
        vec![std::collections::HashMap::new(); t + 1];
    for i in 0..h {
        for j in 0..w {
            if j < w - 1 {
                graph[i * w + j].insert(i * w + j + 1, 1);
                graph[i * w + j + 1].insert(i * w + j, 1);
            }
            if i < h - 1 {
                graph[i * w + j].insert(i * w + j + w, 1);
                graph[i * w + j + w].insert(i * w + j, 1);
            }
            if let Some(v) = test_case[i][j] {
                if v {
                    graph[s].insert(i * w + j, 4);
                    graph[i * w + j].insert(s, 4);
                } else {
                    graph[t].insert(i * w + j, 4);
                    graph[i * w + j].insert(t, 4);
                }
            }
        }
    }
    format!(
        "{}",
        (w - 1) * h + w * (h - 1) - lib::algorithms::maximum_flow(&graph, s, t)
    )
}
