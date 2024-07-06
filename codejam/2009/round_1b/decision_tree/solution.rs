use std::collections::HashSet;

lib::run!();

struct DecisionTree {
    probability: f64,
    decision_point: Option<(String, Box<DecisionTree>, Box<DecisionTree>)>,
}

impl DecisionTree {
    fn from(reversed_tree_lines: &mut Vec<String>) -> DecisionTree {
        let tree_line = reversed_tree_lines
            .pop()
            .unwrap()
            .replace('(', &"")
            .replace(')', &"");
        let tree_line_elements = tree_line.split_whitespace().collect::<Vec<&str>>();
        let mut decision_tree = DecisionTree {
            probability: tree_line_elements[0].parse::<f64>().unwrap(),
            decision_point: None,
        };
        if tree_line_elements.len() == 2 {
            decision_tree.decision_point = Some((
                tree_line_elements[1].to_owned(),
                Box::new(DecisionTree::from(reversed_tree_lines)),
                Box::new(DecisionTree::from(reversed_tree_lines)),
            ));
            reversed_tree_lines.pop();
        }
        decision_tree
    }

    fn get_probability(&self, entry_properties: HashSet<String>) -> f64 {
        self.probability
            * if let Some((property, positive, negative)) = self.decision_point.as_ref() {
                if entry_properties.contains(property) {
                    positive
                } else {
                    negative
                }
                .get_probability(entry_properties)
            } else {
                1f64
            }
    }
}

fn read() -> (DecisionTree, Vec<Vec<String>>) {
    let mut tree_lines: Vec<String> = Vec::new();
    let mut entries: Vec<Vec<String>> = Vec::new();
    lib::input!(usize as l);
    for _ in 0..l {
        lib::input!(String as tree_line);
        tree_lines.push(tree_line);
    }
    tree_lines.reverse();
    let decision_tree = DecisionTree::from(&mut tree_lines);
    lib::input!(usize as a);
    for _ in 0..a {
        lib::input!(Vec<String> as entry);
        entries.push(entry.into_iter().skip(2).collect::<Vec<String>>());
    }
    (decision_tree, entries)
}

fn solve((decision_tree, entries): (DecisionTree, Vec<Vec<String>>)) -> String {
    entries
        .into_iter()
        .map(|entry| {
            format!(
                "\n{:.8}",
                decision_tree.get_probability(HashSet::from_iter(entry.into_iter()))
            )
        })
        .collect()
}
