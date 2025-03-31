lib::run!();

fn read() -> Vec<[f64; 6]> {
    lib::input!(usize as n);
    let mut fs: Vec<[f64; 6]> = Vec::with_capacity(n);
    for _ in 0..n {
        lib::input!([f64; 6] as f);
        fs.push(f);
    }
    fs
}

fn solve(fs: Vec<[f64; 6]>) -> String {
    let n = fs.len() as f64;
    let mut v = [0f64; 6];
    for f in fs {
        for i in 0..6 {
            v[i] += f[i] / n;
        }
    }
    let mut tn = 0f64;
    let mut td = 0f64;
    for i in 0..3 {
        tn -= v[i] * v[i + 3];
        td += v[i + 3].powf(2f64);
    }
    let t = if tn > 0f64 && td > 1e-9 {
        tn / td
    } else {
        0f64
    };
    let mut d = 0f64;
    for i in 0..3 {
        d += (v[i] + t * v[i + 3]).powf(2f64);
    }
    d = d.sqrt();
    format!("{:.9} {:.9}", d, t)
}
