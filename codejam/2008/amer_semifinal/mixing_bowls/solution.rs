use std::collections::HashMap;

lib::run!();

struct TestCase {
    root: String,
    tree: HashMap<String, Vec<String>>,
}

fn read() -> TestCase {
    let insert_node = |row: Vec<String>, tree: &mut HashMap<String, Vec<String>>| {
        tree.insert(
            row[0].clone(),
            row.into_iter()
                .skip(2)
                .filter(|s| s.chars().next().unwrap().is_uppercase())
                .collect::<Vec<String>>(),
        );
    };
    let mut tree: HashMap<String, Vec<String>> = HashMap::new();
    lib::input!(usize as n);
    lib::input!(Vec<String> as row);
    let root = row[0].clone();
    insert_node(row, &mut tree);
    for _ in 1..n {
        lib::input!(Vec<String> as row);
        insert_node(row, &mut tree);
    }
    TestCase { root, tree }
}

fn solve(TestCase { root, tree }: TestCase) -> String {
    format!("{}", minimum_mixing_bowls(&root, &tree))
}

fn minimum_mixing_bowls(root: &String, tree: &HashMap<String, Vec<String>>) -> i64 {
    let mut sub_sols = tree[root]
        .iter()
        .map(|child| minimum_mixing_bowls(child, tree))
        .collect::<Vec<i64>>();
    sub_sols.sort();
    sub_sols.reverse();
    let mut spare_bowls_added = 0i64;
    let mut spare_bowls = 0i64;
    for b in sub_sols.into_iter() {
        spare_bowls_added += std::cmp::max(0, b - spare_bowls);
        spare_bowls = std::cmp::max(spare_bowls, b) - 1;
    }
    spare_bowls_added + if spare_bowls > 0 { 0 } else { 1 }
}
