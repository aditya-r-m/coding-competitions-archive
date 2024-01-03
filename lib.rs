// derived from https://stackoverflow.com/a/57200055/5664000
#[macro_export]
macro_rules! input {
    ($out:ident as Vec<$type:ty>) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out = inner
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
    ($out:ident as [$type:ty; $x:expr]) => {
        lib::input!(vec as Vec<$type>);
        let mut $out: [$type; $x] = [<$type>::default(); $x];
        for i in 0..$x {
            $out[i] = vec[i];
        }
    };
    ($out:ident) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim();
    };
    ($out:ident as $type:ty) => {
        lib::input!(inner);
        let $out = inner.parse::<$type>().expect("Parsable");
    };
}

#[macro_export]
macro_rules! run {
    () => {
        fn main() {
            lib::input!(test_case_count as usize);
            for test_case_index in 1..1 + test_case_count {
                println!("Case #{test_case_index}: {}", solve(read()));
            }
        }
    };
}
