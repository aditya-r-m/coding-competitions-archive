use std::collections::HashSet;

lib::run!();

const L: usize = 10000;

struct TestCase {
    offers: Vec<((usize, usize), String)>,
}

fn read() -> TestCase {
    lib::input!(usize as n);
    let mut offers: Vec<((usize, usize), String)> = Vec::new();
    for _ in 0..n {
        lib::input!((String, usize, usize) as (c, a, b));
        offers.push(((a, b), c));
    }
    TestCase { offers }
}

fn solve(TestCase { mut offers }: TestCase) -> String {
    offers.sort();
    let mut colors = offers
        .iter()
        .map(|(_, color)| color)
        .collect::<Vec<&String>>();
    colors.sort();
    colors.dedup();
    let mut min_accepted_offers = usize::MAX;
    for i in 0..colors.len() {
        for j in i..colors.len() {
            for k in j..colors.len() {
                if let Some(accepted_offers) =
                    find_accepted_offers(&offers, HashSet::from([colors[i], colors[j], colors[k]]))
                {
                    min_accepted_offers = std::cmp::min(min_accepted_offers, accepted_offers);
                }
            }
        }
    }
    if min_accepted_offers < usize::MAX {
        format!("{min_accepted_offers}")
    } else {
        "IMPOSSIBLE".to_owned()
    }
}

fn find_accepted_offers(
    offers: &Vec<((usize, usize), String)>,
    acceptable_colors: HashSet<&String>,
) -> Option<usize> {
    let mut cur_boundary = 1;
    let mut nxt_boundary: Option<usize> = None;
    let mut accepted_offers = 1;
    for &((s, e), _) in offers.iter().filter(|(_, c)| acceptable_colors.contains(c)) {
        if s > cur_boundary {
            if let Some(n) = nxt_boundary {
                cur_boundary = n;
                nxt_boundary = None;
                accepted_offers += 1;
            } else {
                return None;
            }
        }
        if s <= cur_boundary {
            if e < L {
                nxt_boundary = std::cmp::max(nxt_boundary, Some(1 + e));
            } else {
                return Some(accepted_offers);
            }
        }
    }
    None
}
