use lib::collections::ModInt;
use std::collections::BTreeMap;

fn mod_int(i: usize) -> ModInt {
    ModInt::new(i, 10009)
}

lib::run!();

fn read() -> (Vec<Vec<char>>, usize, Vec<Vec<char>>) {
    let polynomial: Vec<Vec<char>>;
    let k: usize;
    lib::input!(Vec<String> as line);
    polynomial = line[0]
        .split(|c| c == '+')
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    k = line[1].parse().unwrap();
    lib::input!(usize as w);
    let mut words: Vec<Vec<char>> = Vec::with_capacity(w);
    for _ in 0..w {
        lib::input!(Vec<char> as word);
        words.push(word);
    }
    (polynomial, k, words)
}

fn solve((polynomial, k, words): (Vec<Vec<char>>, usize, Vec<Vec<char>>)) -> String {
    let word_sketches: Vec<BTreeMap<char, usize>> = words
        .iter()
        .map(|word| {
            word.into_iter().fold(BTreeMap::new(), |mut map, &v| {
                *map.entry(v).or_insert(0usize) += 1usize;
                map
            })
        })
        .collect::<Vec<BTreeMap<char, usize>>>();

    let mut solution = "\u{8}".to_owned();
    for d in 1..=k {
        let mut result = mod_int(0);
        for monomial in polynomial.iter() {
            for mut i in 0..d.pow(monomial.len() as u32) {
                let mut assignment_list: Vec<(usize, usize)> = Vec::new();
                for j in 0..monomial.len() {
                    assignment_list.push((i % d, j));
                    i /= d;
                }
                assignment_list.sort();
                let mut assignments: Vec<BTreeMap<char, usize>> = Vec::new();
                for gi in 0..monomial.len() {
                    if gi == 0 || assignment_list[gi].0 != assignment_list[gi - 1].0 {
                        assignments.push(BTreeMap::new());
                    }
                    *assignments
                        .last_mut()
                        .unwrap()
                        .entry(monomial[assignment_list[gi].1])
                        .or_insert(0usize) += 1;
                }
                let mut submonomial_result =
                    lib::algorithms::exp(mod_int(word_sketches.len()), d - assignments.len());
                for assignment in assignments {
                    let mut assignment_result = mod_int(0usize);
                    for word_sketch in word_sketches.iter() {
                        let mut word_result = mod_int(1usize);
                        for (char, p) in assignment.iter() {
                            word_result *= lib::algorithms::exp(
                                mod_int(*word_sketch.get(char).unwrap_or(&0usize)),
                                *p,
                            );
                        }
                        assignment_result += word_result;
                    }
                    submonomial_result *= assignment_result;
                }
                result += submonomial_result;
            }
        }
        solution = format!("{solution} {}", result.i);
    }
    solution
}
