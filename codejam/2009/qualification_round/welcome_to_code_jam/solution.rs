lib::run!();

fn read() -> Vec<char> {
    lib::input!(Vec<char> as string);
    string
}

fn solve(string: Vec<char>) -> String {
    let pattern: Vec<char> = "welcome to code jam".chars().collect::<Vec<char>>();
    let mut cache = vec![0; 1 + pattern.len()];
    cache[0] = 1;
    for c in string.into_iter() {
        for (i, &p) in pattern.iter().enumerate().rev() {
            if p == c {
                cache[i + 1] += cache[i];
                cache[i + 1] %= 10000;
            }
        }
    }
    format!("{:0>4}", cache.into_iter().last().unwrap())
}
