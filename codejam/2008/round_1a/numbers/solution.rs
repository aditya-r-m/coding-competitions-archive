lib::run!();

fn read() -> usize {
    lib::input!(usize as n);
    n
}

fn solve(n: usize) -> String {
    let mod_int = |i| lib::collections::ModInt::new(i, 1000);
    let lib::collections::ModInt { i: v, m: _ } = mod_int(999)
        + mod_int(2)
            * lib::algorithms::exp(
                lib::collections::SquareMatrix::new(vec![
                    vec![mod_int(3), mod_int(5)],
                    vec![mod_int(1), mod_int(3)],
                ]),
                n,
            )
            .rows[0][0];
    format!("{:0>3}", v)
}
