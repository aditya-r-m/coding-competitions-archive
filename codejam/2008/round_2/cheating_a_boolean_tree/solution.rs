lib::run!();

enum Node {
    And(bool),
    Or(bool),
    Leaf(bool),
}

struct TestCase {
    v: bool,
    nodes: Vec<Node>,
}

fn read() -> TestCase {
    lib::input!([usize; 2] as [m, v]);
    let mut nodes: Vec<Node> = Vec::with_capacity(m);
    for _ in 0..m >> 1 {
        lib::input!([usize; 2] as [g, c]);
        nodes.push(match g {
            0 => Node::Or(c > 0),
            1 => Node::And(c > 0),
            _ => unreachable!(),
        });
    }
    for _ in 0..1 + (m >> 1) {
        lib::input!(usize as i);
        nodes.push(Node::Leaf(i > 0));
    }
    TestCase { v: v > 0, nodes }
}

fn solve(TestCase { v, nodes }: TestCase) -> String {
    let min = |oa: Option<usize>, ob: Option<usize>| std::cmp::min(oa.or(ob), ob.or(oa));
    let add = |oa: Option<usize>, ob: Option<usize>| {
        if let Some(a) = oa {
            if let Some(b) = ob {
                return Some(a + b);
            }
        }
        None
    };

    let mut minimum_cost: std::collections::HashMap<bool, Vec<Option<usize>>> =
        std::collections::HashMap::from([
            (false, vec![None; nodes.len()]),
            (true, vec![None; nodes.len()]),
        ]);

    for (l, leaf) in nodes.iter().enumerate().skip(nodes.len() >> 1) {
        match leaf {
            Node::Leaf(input) => {
                minimum_cost.get_mut(input).unwrap()[l] = Some(0);
            }
            _ => {
                unreachable!();
            }
        }
    }

    for (n, node) in nodes.iter().enumerate().take(nodes.len() >> 1).rev() {
        let left_true_cost = minimum_cost[&true][1 + (n << 1)];
        let left_false_cost = minimum_cost[&false][1 + (n << 1)];
        let right_true_cost = minimum_cost[&true][2 + (n << 1)];
        let right_false_cost = minimum_cost[&false][2 + (n << 1)];
        match node {
            Node::And(changeable) => {
                minimum_cost.get_mut(&true).unwrap()[n] = add(left_true_cost, right_true_cost);
                minimum_cost.get_mut(&false).unwrap()[n] = min(left_false_cost, right_false_cost);
                if *changeable {
                    minimum_cost.get_mut(&true).unwrap()[n] = min(
                        minimum_cost[&true][n],
                        add(Some(1), min(left_true_cost, right_true_cost)),
                    );
                }
            }
            Node::Or(changeable) => {
                minimum_cost.get_mut(&true).unwrap()[n] = min(left_true_cost, right_true_cost);
                minimum_cost.get_mut(&false).unwrap()[n] = add(left_false_cost, right_false_cost);
                if *changeable {
                    minimum_cost.get_mut(&false).unwrap()[n] = min(
                        minimum_cost[&false][n],
                        add(Some(1), min(left_false_cost, right_false_cost)),
                    );
                }
            }
            _ => {
                unreachable!();
            }
        }
    }

    if let Some(s) = minimum_cost[&v][0] {
        format!("{s}")
    } else {
        "IMPOSSIBLE".to_string()
    }
}
