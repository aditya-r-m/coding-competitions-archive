use std::collections::{HashSet, VecDeque};

lib::run!();

fn read() -> (Vec<Vec<bool>>, Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut initial_state: Vec<(usize, usize)> = Vec::new();
    let mut terminal_state: Vec<(usize, usize)> = Vec::new();
    lib::input!([usize; 2] as [r, c]);
    grid.push(vec![true; c + 2]);
    for i in 1..=r {
        lib::input!(String as mut row);
        row.insert(0, '#');
        row.push('#');
        grid.push(Vec::new());
        for (j, c) in row.chars().enumerate() {
            grid.last_mut().unwrap().push(c == '#');
            if c == 'o' || c == 'w' {
                initial_state.push((i, j));
            }
            if c == 'x' || c == 'w' {
                terminal_state.push((i, j));
            }
        }
    }
    grid.push(vec![true; c + 2]);
    initial_state.sort();
    terminal_state.sort();
    (grid, initial_state, terminal_state)
}

fn solve(
    (grid, initial_state, terminal_state): (
        Vec<Vec<bool>>,
        Vec<(usize, usize)>,
        Vec<(usize, usize)>,
    ),
) -> String {
    let mut distance = 0;
    let mut queue: Vec<Vec<(usize, usize)>> = vec![initial_state.clone(); 1];
    let mut visited: HashSet<Vec<(usize, usize)>> = HashSet::from([initial_state]);
    while queue.len() > 0 {
        let mut next_queue: Vec<Vec<(usize, usize)>> = Vec::new();
        for state in queue.into_iter() {
            if state == terminal_state {
                return format!("{distance}");
            }
            for next_state in get_next_states(state, &grid) {
                if !visited.contains(&next_state) {
                    visited.insert(next_state.clone());
                    next_queue.push(next_state);
                }
            }
        }
        queue = next_queue;
        distance += 1;
    }
    "-1".to_owned()
}

fn get_next_states(state: Vec<(usize, usize)>, grid: &Vec<Vec<bool>>) -> Vec<Vec<(usize, usize)>> {
    let is_state_stable = is_stable(&state);
    let state_set: HashSet<(usize, usize)> = HashSet::from_iter(state.iter().cloned());
    let mut next_states: Vec<Vec<(usize, usize)>> = Vec::new();
    for i in 0..state.len() {
        for diff in [
            (-1isize, 0isize),
            (1isize, 0isize),
            (0isize, -1isize),
            (0isize, 1isize),
        ] {
            let mut next_state = state.clone();
            next_state[i].0 = next_state[i].0.checked_add_signed(diff.0).unwrap();
            next_state[i].1 = next_state[i].1.checked_add_signed(diff.1).unwrap();
            let push_point = (
                state[i].0.checked_add_signed(-diff.0).unwrap(),
                state[i].1.checked_add_signed(-diff.1).unwrap(),
            );
            if (is_state_stable || is_stable(&next_state))
                && !grid[next_state[i].0][next_state[i].1]
                && !grid[push_point.0][push_point.1]
                && !state_set.contains(&next_state[i])
                && !state_set.contains(&push_point)
            {
                next_state.sort();
                next_states.push(next_state);
            }
        }
    }
    next_states.sort();
    next_states.dedup();
    next_states
}

fn is_stable(state: &Vec<(usize, usize)>) -> bool {
    let state_set: HashSet<(usize, usize)> = HashSet::from_iter(state.iter().cloned());
    let mut visited: HashSet<(usize, usize)> = HashSet::from([state[0]]);
    let mut queue: VecDeque<(usize, usize)> = VecDeque::from([state[0]]);
    while let Some(block) = queue.pop_back() {
        for diff in [
            (-1isize, 0isize),
            (1isize, 0isize),
            (0isize, -1isize),
            (0isize, 1isize),
        ] {
            let next_block = (
                block.0.checked_add_signed(diff.0).unwrap_or(0usize),
                block.1.checked_add_signed(diff.1).unwrap_or(0usize),
            );
            if state_set.contains(&next_block) && !visited.contains(&next_block) {
                queue.push_back(next_block);
                visited.insert(next_block);
            }
        }
    }
    state_set.len() == visited.len()
}
