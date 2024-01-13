lib::run!();

struct TestCase {
    k: usize,
    frequencies: Vec<usize>,
}

fn read() -> TestCase {
    lib::input!([usize; 3] as [_, k, _]);
    lib::input!(Vec<usize> as frequencies);
    TestCase { k, frequencies }
}

fn solve(TestCase { k, mut frequencies }: TestCase) -> String {
    frequencies.sort();
    frequencies.reverse();
    format!(
        "{}",
        frequencies
            .into_iter()
            .enumerate()
            .map(|(i, f)| f * (1 + i / k))
            .sum::<usize>()
    )
}
