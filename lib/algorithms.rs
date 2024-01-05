pub fn mod_exp(a: usize, p: usize, m: usize) -> usize {
    if p == 0 {
        1
    } else if p & 1 == 0 {
        mod_exp((a * a) % m, p >> 1, m)
    } else {
        (a * mod_exp(a, p - 1, m)) % m
    }
}

pub fn is_probable_prime(n: usize) -> bool {
    let bases: [usize; 7] = [2, 3, 5, 7, 11, 13, 17];
    if n <= 17 {
        return bases.contains(&n);
    }
    let mut s = 0;
    let mut d = n - 1;
    while d & 1 == 0 {
        d >>= 1;
        s += 1;
    }
    for base in bases {
        let mut x = mod_exp(base, d, n);
        for _ in 0..s {
            x = {
                let y = (x * x) % n;
                if y == 1 && x != 1 && x != n - 1 {
                    return false;
                }
                y
            }
        }
        if x != 1 {
            return false;
        }
    }
    true
}
