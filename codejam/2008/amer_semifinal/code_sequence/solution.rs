lib::run!();

const M: usize = 10007usize;

fn read() -> Vec<usize> {
    lib::input!(usize as _l);
    lib::input!(Vec<usize> as sequence);
    sequence
}

fn solve(mut seq: Vec<usize>) -> String {
    seq.reverse();
    let mut diffs = Vec::new();
    for i in 0..seq.len() - 1 {
        diffs.push((seq[i] + (M - seq[i + 1])) % M);
    }
    if let Some(d) = previous_diff(diffs) {
        format!("{}", (seq[0] + d) % M)
    } else {
        "UNKNOWN".to_string()
    }
}

fn previous_diff(diffs: Vec<usize>) -> Option<usize> {
    if diffs.len() < 3 {
        return None;
    }
    let mut odd_diffs: Vec<usize> = Vec::new();
    for i in (1..diffs.len()).step_by(2) {
        odd_diffs.push(diffs[i]);
    }
    let mut varying_even_diffs = false;
    for i in (0..diffs.len() - 2).step_by(2) {
        varying_even_diffs |= diffs[i] != diffs[i + 2];
    }
    if varying_even_diffs {
        Some(odd_diffs[0])
    } else {
        previous_diff(odd_diffs)
    }
}
