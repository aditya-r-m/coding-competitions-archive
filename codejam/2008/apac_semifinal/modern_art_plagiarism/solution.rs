use std::collections::HashMap;

lib::run!();

struct TestCase {
    large_adjacency_list: Vec<Vec<usize>>,
    small_adjacency_list: Vec<Vec<usize>>,
}

fn read() -> TestCase {
    lib::input!(usize as size_large);
    let mut large_adjacency_list: Vec<Vec<usize>> = vec![Vec::new(); size_large];
    for _ in 0..size_large - 1 {
        lib::input!([usize; 2] as [u, v]);
        large_adjacency_list[u - 1].push(v - 1);
        large_adjacency_list[v - 1].push(u - 1);
    }
    lib::input!(usize as size_small);
    let mut small_adjacency_list: Vec<Vec<usize>> = vec![Vec::new(); size_small];
    for _ in 0..size_small - 1 {
        lib::input!([usize; 2] as [u, v]);
        small_adjacency_list[u - 1].push(v - 1);
        small_adjacency_list[v - 1].push(u - 1);
    }
    TestCase {
        large_adjacency_list,
        small_adjacency_list,
    }
}

fn get_rooted_tree(adjacency_list: &[Vec<usize>], root: usize) -> Vec<Vec<usize>> {
    let mut rooted_tree = vec![Vec::new(); adjacency_list.len()];
    let mut q = std::collections::VecDeque::new();
    q.push_back(root);
    while let Some(u) = q.pop_front() {
        for &v in adjacency_list[u].iter() {
            if rooted_tree[v].is_empty() {
                rooted_tree[u].push(v);
                q.push_back(v);
            }
        }
    }
    rooted_tree
}

fn is_isomorphic(
    large_tree: &[Vec<usize>],
    small_tree: &[Vec<usize>],
    large_root: usize,
    small_root: usize,
) -> bool {
    let l = large_tree[large_root].len();
    let r = small_tree[small_root].len();
    let mut graph: Vec<HashMap<usize, usize>> = vec![HashMap::new(); l + r + 2];
    let s = graph.len() - 2;
    let t = graph.len() - 1;
    for (iu, &u) in large_tree[large_root].iter().enumerate() {
        graph[s].insert(iu, 1);
        for (iv, &v) in small_tree[small_root].iter().enumerate() {
            if is_isomorphic(large_tree, small_tree, u, v) {
                graph[iu].insert(l + iv, 1);
                graph[l + iv].insert(t, 1);
            }
        }
    }
    lib::algorithms::maximum_flow(&graph, s, t) == r
}

fn solve(
    TestCase {
        large_adjacency_list,
        small_adjacency_list,
    }: TestCase,
) -> String {
    let large_tree = get_rooted_tree(&large_adjacency_list, 0);
    for j in 0..small_adjacency_list.len() {
        let small_tree = get_rooted_tree(&small_adjacency_list, j);
        for i in 0..large_adjacency_list.len() {
            if is_isomorphic(&large_tree, &small_tree, i, j) {
                return "YES".to_owned();
            }
        }
    }
    "NO".to_owned()
}
