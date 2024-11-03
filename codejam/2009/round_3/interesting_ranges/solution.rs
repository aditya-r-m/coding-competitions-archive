use lib::algorithms::exp;
use lib::collections::ModInt;

fn mod_int(i: u32) -> ModInt {
    ModInt {
        i: i as usize,
        m: 1_000_000_007,
    }
}

lib::run!();

fn read() -> [Vec<u32>; 2] {
    lib::input!(Vec<char> as range);
    let mut points = range.split(|&c| c == ' ');
    [
        points
            .next()
            .unwrap()
            .into_iter()
            .map(|&c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>(),
        points
            .next()
            .unwrap()
            .into_iter()
            .map(|&c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>(),
    ]
}

fn solve([mut l, r]: [Vec<u32>; 2]) -> String {
    let e0l: ModInt;
    let o0l: ModInt;
    let e0r: ModInt = e0_range_count(&r);
    let o0r: ModInt = o0_range_count(&r);
    if l == vec![1] {
        e0l = mod_int(0);
        o0l = mod_int(0);
    } else {
        for _ in 0..2 {
            for i in (0..l.len()).rev() {
                if l[i] > 0 {
                    l[i] -= 1;
                    break;
                }
                l[i] = 9;
            }
        }
        if l[0] == 0 {
            l.remove(0);
        }
        e0l = e0_range_count(&l);
        o0l = o0_range_count(&l);
    }
    let e = e0r - e0l;
    let o = o0r - o0l;
    format!(
        "{}",
        (e * (e - mod_int(1)) / mod_int(2) + o * (o - mod_int(1)) / mod_int(2)).i
    )
}

fn o0_range_count(n: &Vec<u32>) -> ModInt {
    let mut result = mod_int(0);
    for i in 0..n.len() {
        result *= mod_int(10);
        result += mod_int(n[i]);
    }
    result + mod_int(1) - e0_range_count(n)
}

fn e0_range_count(n: &Vec<u32>) -> ModInt {
    let l = n.len();
    if l == 0 {
        return mod_int(0);
    }
    if l == 1 {
        return mod_int(e0_range_count_unoptimized(n[0]));
    }
    if l == 2 {
        return mod_int(e0_range_count_unoptimized(n[0] * 10 + n[1]));
    }
    let mut result = mod_int(e0_range_count_unoptimized(100));
    for w in 3..l {
        result += (exp(mod_int(10), (w - 1) >> 1) - exp(mod_int(10), ((w - 1) >> 1) - 1))
            * exp(mod_int(11), 1 - (w & 1))
            * mod_int(5)
            * exp(mod_int(10), (w - 1) >> 1);
    }
    if n[0] == 1 && n.iter().skip(1).all(|&d| d == 0) {
        return result;
    }
    let mut p = n.clone();
    for i in 0..l >> 1 {
        if p[i] > p[l - 1 - i] {
            for j in (0..l - 1 - i).rev() {
                if p[j] > 0 {
                    p[j] -= 1;
                    break;
                } else {
                    p[j] = 9;
                }
            }
        }
        p[l - 1 - i] = p[i];
    }
    let mut m = mod_int(0);
    for i in 0..(l - 1) >> 1 {
        m *= mod_int(10);
        m += mod_int(p[i]);
    }
    result += (m - exp(mod_int(10), ((l - 1) >> 1) - 1))
        * exp(mod_int(11), 1 - (l & 1))
        * mod_int(5)
        * exp(mod_int(10), (l - 1) >> 1);
    result += exp(mod_int(11), 1 - (l & 1))
        * mod_int((p[(l - 1) >> 1] + 1) >> 1)
        * exp(mod_int(10), (l - 1) >> 1);
    if p[(l - 1) >> 1] & 1 == 0 {
        let mut nv = mod_int(0);
        let mut pv = mod_int(0);
        for i in 0..l {
            nv *= mod_int(10);
            nv += mod_int(n[i]);
            pv *= mod_int(10);
            pv += mod_int(p[i]);
        }
        result += mod_int(1) + nv - pv;
    }
    result
}

fn e0_range_count_unoptimized(n: u32) -> u32 {
    let mut r = 0u32;
    let mut e = false;
    for i in 1..=n {
        if format!("{i}") == format!("{i}").chars().rev().collect::<String>() {
            e = !e;
        }
        if e {
            r += 1;
        }
    }
    r
}
