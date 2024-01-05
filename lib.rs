// ref: https://stackoverflow.com/a/57200055/5664000

#[macro_export]
macro_rules! input {
    ($out:ident) => {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("A String");
        let $out = buffer.trim();
    };
    (Vec<$type:ty> as $out:ident) => {
        lib::input!(buffer);
        let $out = buffer
            .split_whitespace()
            .map(|s| s.parse::<$type>().expect("Parsable"))
            .collect::<Vec<$type>>();
    };
    ($type:ty as $out:ident) => {
        lib::input!(buffer);
        let $out = buffer.parse::<$type>().expect("Parsable");
    };
    ([$type:ty; $x:expr] as $out:pat_param) => {
        lib::input!(Vec<$type> as vector);
        let mut buffer: [$type; $x] = [<$type>::default(); $x];
        for i in 0..$x {
            buffer[i] = vector[i];
        }
        let $out = buffer;
    };
}

#[macro_export]
macro_rules! run {
    () => {
        fn main() {
            lib::input!(usize as test_case_count);
            for test_case_index in 1..1 + test_case_count {
                println!("Case #{test_case_index}: {}", solve(read()));
            }
        }
    };
}
