fn main() {
    lib::input!([usize; 3] as [_, d, n]);
    let mut words: Vec<Vec<char>> = Vec::new();
    for _ in 0..d {
        lib::input!(Vec<char> as word);
        words.push(word);
    }
    for c in 1..1 + n {
        lib::input!(Vec<char> as pattern_input);
        let mut i = 0usize;
        let mut pattern: Vec<std::collections::HashSet<char>> = Vec::new();
        while i < pattern_input.len() {
            if pattern_input[i] == '(' {
                pattern.push(std::collections::HashSet::new());
                while pattern_input[i] != ')' {
                    pattern.last_mut().unwrap().insert(pattern_input[i]);
                    i += 1;
                }
            } else {
                pattern.push(std::collections::HashSet::from([pattern_input[i]]));
            }
            i += 1;
        }
        println!(
            "Case #{c}: {}",
            words
                .iter()
                .filter(|word| word.iter().zip(pattern.iter()).all(|(c, s)| s.contains(&c)))
                .count()
        );
    }
}
