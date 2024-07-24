lib::run!();

fn read() -> Vec<char> {
    lib::input!(Vec<char> as digits);
    digits
}

fn solve(mut digits: Vec<char>) -> String {
    for i in (0..digits.len()).rev() {
        for j in (i + 1..digits.len()).rev() {
            if digits[i] < digits[j] {
                (digits[i], digits[j]) = (digits[j], digits[i]);
                digits[i + 1..].sort();
                return digits.into_iter().collect();
            }
        }
    }
    digits.push('0');
    digits.sort();
    for i in 0..digits.len() {
        if digits[i] > '0' {
            (digits[0], digits[i]) = (digits[i], digits[0]);
            return digits.into_iter().collect();
        }
    }
    unreachable!()
}
