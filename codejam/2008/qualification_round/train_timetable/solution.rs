lib::run!();

struct TestCase {
    turnaround_time: usize,
    trains_a_to_b: Vec<(usize, usize)>,
    trains_b_to_a: Vec<(usize, usize)>,
}

fn read() -> TestCase {
    lib::input!(usize as turnaround_time);
    lib::input!([usize; 2] as [count_a_to_b, count_b_to_a]);
    let mut trains_a_to_b: Vec<(usize, usize)> = Vec::new();
    let mut trains_b_to_a: Vec<(usize, usize)> = Vec::new();
    for i in 0..count_a_to_b + count_b_to_a {
        lib::input!(Vec<String> as schedule);
        let start_time = parse(&schedule[0]);
        let end_time = parse(&schedule[1]);
        if i < count_a_to_b {
            trains_a_to_b.push((start_time, end_time));
        } else {
            trains_b_to_a.push((start_time, end_time));
        }
    }
    TestCase {
        turnaround_time,
        trains_a_to_b,
        trains_b_to_a,
    }
}

fn parse(clock_time: &str) -> usize {
    let clock_time_chars = clock_time.chars().collect::<Vec<char>>();
    (clock_time_chars[0].to_digit(10).unwrap() * 600
        + clock_time_chars[1].to_digit(10).unwrap() * 60
        + clock_time_chars[3].to_digit(10).unwrap() * 10
        + clock_time_chars[4].to_digit(10).unwrap()) as usize
}

fn solve(
    TestCase {
        turnaround_time,
        trains_a_to_b,
        trains_b_to_a,
    }: TestCase,
) -> String {
    let mut departures_a: Vec<(usize, i64)> = Vec::new();
    let mut departures_b: Vec<(usize, i64)> = Vec::new();
    for (start_time, end_time) in trains_a_to_b.into_iter() {
        departures_a.push((start_time, 1));
        departures_b.push((end_time + turnaround_time, -1));
    }
    for (start_time, end_time) in trains_b_to_a.into_iter() {
        departures_b.push((start_time, 1));
        departures_a.push((end_time + turnaround_time, -1));
    }
    departures_a.sort();
    departures_b.sort();
    let mut trains_departed_a = 0i64;
    let mut trains_departed_b = 0i64;
    let mut trains_starting_a = 0i64;
    let mut trains_starting_b = 0i64;
    for (_, sign) in departures_a.into_iter() {
        trains_departed_a += sign;
        trains_starting_a = std::cmp::max(trains_starting_a, trains_departed_a);
    }
    for (_, sign) in departures_b.into_iter() {
        trains_departed_b += sign;
        trains_starting_b = std::cmp::max(trains_starting_b, trains_departed_b);
    }
    format!("{} {}", trains_starting_a, trains_starting_b)
}
