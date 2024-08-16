use std::collections::HashMap;

lib::run!();

fn read() -> Vec<char> {
    lib::input!(Vec<char> as number);
    number
}

fn solve(number: Vec<char>) -> String {
    let mut base = 2;
    let mut cloned_number = number.clone();
    let mut digit_value_map: HashMap<char, usize> = HashMap::new();
    cloned_number.dedup();
    digit_value_map.insert(cloned_number[0], 1);
    if cloned_number.len() > 1 {
        digit_value_map.insert(cloned_number[1], 0);
        cloned_number.sort();
        cloned_number.dedup();
        base = cloned_number.len();
    }
    let mut number_value = 0usize;
    let mut place_value = base.pow(number.len() as u32 - 1);
    for digit in number.into_iter() {
        let next_digit_value = digit_value_map.len();
        let digit_value = *digit_value_map.entry(digit).or_insert(next_digit_value);
        number_value += digit_value * place_value;
        place_value /= base;
    }
    format!("{number_value}")
}
