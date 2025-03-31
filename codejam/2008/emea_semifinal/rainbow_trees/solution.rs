use lib::collections::ModInt;

const M: usize = 1_000_000_009;
fn mod_int(i: usize) -> ModInt {
    ModInt::new(i, M)
}

lib::run!();

struct TestCase {
    k: usize,
    adjacency_list: Vec<Vec<usize>>,
}

fn read() -> TestCase {
    lib::input!([usize; 2] as [n, k]);
    let mut adjacency_list = vec![Vec::new(); n];
    for _ in 1..n {
        lib::input!([usize; 2] as [x, y]);
        adjacency_list[x - 1].push(y - 1);
        adjacency_list[y - 1].push(x - 1);
    }
    TestCase { k, adjacency_list }
}

fn solve(TestCase { k, adjacency_list }: TestCase) -> String {
    format!("{}", dfs(0, 0, 0, k, &adjacency_list).i)
}

fn dfs(
    node: usize,
    parent: usize,
    parent_degree: usize,
    k: usize,
    adjacency_list: &Vec<Vec<usize>>,
) -> ModInt {
    let mut accumulator = mod_int(1);
    for (c, &child) in adjacency_list[node]
        .iter()
        .filter(|&&child| child != parent)
        .enumerate()
    {
        accumulator *= mod_int(k - std::cmp::min(k, parent_degree + c));
        accumulator *= dfs(child, node, adjacency_list[node].len(), k, adjacency_list);
    }
    accumulator
}
