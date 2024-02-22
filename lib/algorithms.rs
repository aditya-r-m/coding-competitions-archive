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

struct FlowGraph {
    capacity: Vec<HashMap<usize, i64>>,
    flow: Vec<HashMap<usize, i64>>,
    labels: Vec<usize>,
    excess: Vec<i64>,
    linked_arcs: Vec<Vec<usize>>,
    current_arc: Vec<usize>,
}

impl FlowGraph {
    fn new(graph: Vec<HashMap<usize, i64>>) -> FlowGraph {
        let mut capacity_graph = graph.clone();
        let mut empty_flow = vec![HashMap::new(); graph.len()];
        let mut linked_arcs = vec![Vec::new(); graph.len()];
        for (u, edges) in graph.iter().enumerate() {
            for &v in edges.keys() {
                capacity_graph[u].insert(v, graph[u][&v]);
                if !graph[v].contains_key(&u) {
                    capacity_graph[v].insert(u, 0);
                }
                empty_flow[u].insert(v, 0);
                empty_flow[v].insert(u, 0);
                linked_arcs[u].push(v);
                linked_arcs[v].push(u);
            }
        }
        FlowGraph {
            capacity: capacity_graph,
            flow: empty_flow,
            labels: vec![0; graph.len()],
            excess: vec![0; graph.len()],
            linked_arcs,
            current_arc: vec![0; graph.len()],
        }
    }

    fn admissible_flow(&self, u: usize, v: usize) -> i64 {
        if self.labels[u] <= self.labels[v] {
            0
        } else {
            std::cmp::min(self.excess[u], self.capacity[u][&v] - self.flow[u][&v])
        }
    }

    fn push(&mut self, u: usize, v: usize) -> bool {
        let f = self.admissible_flow(u, v);
        self.excess[u] -= f;
        self.excess[v] += f;
        *self.flow[u].get_mut(&v).expect("link") += f;
        *self.flow[v].get_mut(&u).expect("link") -= f;
        f > 0
    }

    fn relabel(&mut self, u: usize) {
        self.labels[u] = 1 + self.linked_arcs[u]
            .iter()
            .map(|&v| {
                if self.capacity[u][&v] - self.flow[u][&v] > 0 {
                    self.labels[v]
                } else {
                    usize::MAX
                }
            })
            .reduce(std::cmp::min)
            .expect("non-empty");
    }

    fn discharge(&mut self, u: usize, queue: &mut VecDeque<usize>, queue_set: &mut HashSet<usize>) {
        while self.excess[u] > 0 {
            while self.current_arc[u] < self.linked_arcs[u].len() {
                let v = self.linked_arcs[u][self.current_arc[u]];
                if self.push(u, v) && !queue_set.contains(&v) {
                    queue.push_back(v);
                    queue_set.insert(v);
                }
                if self.admissible_flow(u, v) == 0 {
                    self.current_arc[u] += 1;
                }
            }
            self.current_arc[u] = 0;
            self.relabel(u)
        }
    }
}

pub fn maximum_flow(graph: &[HashMap<usize, usize>], s: usize, t: usize) -> usize {
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut queue_set: HashSet<usize> = HashSet::new();
    let mut flow_graph = FlowGraph::new(
        graph
            .iter()
            .map(|m| {
                m.iter()
                    .map(|(&k, &v)| (k, v as i64))
                    .collect::<HashMap<usize, i64>>()
            })
            .collect::<Vec<HashMap<usize, i64>>>(),
    );
    flow_graph.labels[s] = graph.len();
    flow_graph.excess[s] = i64::MAX;
    queue_set.insert(s);
    queue_set.insert(t);
    while flow_graph.current_arc[s] < flow_graph.linked_arcs[s].len() {
        let v = flow_graph.linked_arcs[s][flow_graph.current_arc[s]];
        if flow_graph.push(s, v) {
            queue.push_back(v);
            queue_set.insert(v);
        }
        flow_graph.current_arc[s] += 1;
        flow_graph.excess[s] = i64::MAX;
    }
    while let Some(u) = queue.pop_front() {
        queue_set.remove(&u);
        flow_graph.discharge(u, &mut queue, &mut queue_set);
    }
    flow_graph.excess[t] as usize
}
