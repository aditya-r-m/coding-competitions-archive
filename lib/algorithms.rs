use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::ops::Mul;

pub fn exp<T>(a: T, p: usize) -> T
where
    T: Clone + crate::collections::MulIdentity + Mul<Output = T>,
{
    if p == 0 {
        a.mul_identity()
    } else if p & 1 == 0 {
        exp(a.clone() * a, p >> 1)
    } else {
        a.clone() * exp(a, p - 1)
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
        let crate::collections::ModInt { i: mut x, m: _ } =
            exp(crate::collections::ModInt::new(base, n), d);
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

pub fn maximum_flow(graph: &[HashMap<usize, usize>], s: usize, t: usize) -> usize {
    let mut linked_arcs: Vec<Vec<usize>> = vec![Vec::new(); graph.len()];
    let mut capacity: Vec<HashMap<usize, i64>> = vec![HashMap::new(); graph.len()];
    let mut flow: Vec<HashMap<usize, i64>> = vec![HashMap::new(); graph.len()];
    for (u, edges) in graph.iter().enumerate() {
        for (&v, &w) in edges.iter() {
            linked_arcs[u].push(v);
            capacity[u].insert(v, w as i64);
            flow[u].insert(v, 0);
            if !graph[v].contains_key(&u) {
                linked_arcs[v].push(u);
                capacity[v].insert(u, 0);
                flow[v].insert(u, 0);
            }
        }
    }
    let mut current_arc = vec![0usize; graph.len()];
    let mut labels = vec![0usize; graph.len()];
    let mut excess = vec![0i64; graph.len()];
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut queue_set: HashSet<usize> = HashSet::new();
    labels[s] = graph.len();
    queue_set.insert(s);
    queue_set.insert(t);
    for &v in linked_arcs[s].iter() {
        let admissible_flow = capacity[s].get(&v).unwrap();
        *flow[s].get_mut(&v).unwrap() += admissible_flow;
        *flow[v].get_mut(&s).unwrap() -= admissible_flow;
        excess[v] += admissible_flow;
        queue.push_back(v);
        queue_set.insert(v);
    }
    while let Some(u) = queue.pop_front() {
        while excess[u] > 0 {
            if current_arc[u] == linked_arcs[u].len() {
                current_arc[u] = 0;
                labels[u] = 1 + linked_arcs[u]
                    .iter()
                    .map(|&v| labels[v])
                    .filter(|&l| labels[u] <= l)
                    .reduce(std::cmp::min)
                    .unwrap();
            }
            let v = linked_arcs[u][current_arc[u]];
            if labels[u] <= labels[v] {
                current_arc[u] += 1;
                continue;
            }
            if capacity[u][&v] > flow[u][&v] {
                let admissible_flow = std::cmp::min(excess[u], capacity[u][&v] - flow[u][&v]);
                *flow[u].get_mut(&v).unwrap() += admissible_flow;
                *flow[v].get_mut(&u).unwrap() -= admissible_flow;
                excess[u] -= admissible_flow;
                excess[v] += admissible_flow;
                if !queue_set.contains(&v) {
                    queue.push_back(v);
                    queue_set.insert(v);
                }
            }
            if capacity[u][&v] == flow[u][&v] {
                current_arc[u] += 1;
            }
        }
        queue_set.remove(&u);
    }
    excess[t] as usize
}
