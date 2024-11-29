use std::collections::{BTreeMap, BTreeSet, VecDeque};

lib::run!();

fn read() -> Vec<String> {
    lib::input!(usize as _);
    lib::input!(Vec<String> as colors);
    colors
}

fn solve(mut colors: Vec<String>) -> String {
    colors.insert(0, "_".to_owned());
    colors.push("_".to_owned());
    let position_pairs = get_position_pairs(colors);
    let link_graph: Vec<BTreeSet<usize>>;
    if let Some(bipartite_link_graph) = get_bipartite_link_graph(&position_pairs) {
        link_graph = bipartite_link_graph;
    } else {
        return format!("-1");
    }
    let containment_graph = get_containment_graph(&position_pairs);
    let (component_map, components) = get_components(&link_graph);
    let mut result: Option<isize> = None;
    let mut cache: BTreeMap<(usize, isize), Option<isize>> = BTreeMap::new();
    for i in 0isize..=position_pairs.len() as isize {
        let current_result = get_lowest_solvable_depth(
            &position_pairs,
            0,
            i,
            &component_map,
            &components,
            &containment_graph,
            &mut cache,
        )
        .map(|d| d - i);
        result = std::cmp::max(result, current_result);
    }
    if let Some(v) = result {
        format!("{}", -v - 1)
    } else {
        format!("-1")
    }
}

fn get_position_pairs(colors: Vec<String>) -> Vec<(usize, usize)> {
    let mut position_pairs: Vec<(usize, usize)> = Vec::new();
    let mut color_locations = colors
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<Vec<(String, usize)>>();
    color_locations.sort();
    for i in (0..color_locations.len()).step_by(2) {
        position_pairs.push((color_locations[i].1, color_locations[i + 1].1));
    }
    position_pairs.sort();
    position_pairs
}

fn is_linked((us, ue): (usize, usize), (vs, ve): (usize, usize)) -> bool {
    if us == vs {
        false
    } else if us < vs {
        vs < ue && ue < ve
    } else {
        is_linked((vs, ve), (us, ue))
    }
}

fn is_contained((us, ue): (usize, usize), (vs, ve): (usize, usize)) -> bool {
    us < vs && ve < ue
}

fn get_bipartite_link_graph(position_pairs: &Vec<(usize, usize)>) -> Option<Vec<BTreeSet<usize>>> {
    let mut link_graph: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); position_pairs.len()];
    for (iu, &u) in position_pairs.iter().enumerate() {
        for (iv, &v) in position_pairs.iter().enumerate().skip(iu + 1) {
            if !is_linked(u, v) {
                continue;
            }
            link_graph[iu].insert(iv);
            link_graph[iv].insert(iu);
            for &w in position_pairs.iter().skip(iv + 1) {
                if is_linked(u, w) && is_linked(v, w) {
                    return None;
                }
            }
        }
    }
    Some(link_graph)
}

fn get_containment_graph(position_pairs: &Vec<(usize, usize)>) -> Vec<BTreeSet<usize>> {
    let mut containment_graph: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); position_pairs.len()];
    for (iu, &u) in position_pairs.iter().enumerate() {
        for (iv, &v) in position_pairs.iter().enumerate().skip(iu + 1) {
            if is_contained(u, v) {
                containment_graph[iu].insert(iv);
            }
            if is_contained(v, u) {
                containment_graph[iv].insert(iu);
            }
        }
    }
    let mut trimmed_containment_graph = containment_graph.clone();
    for iu in 0..containment_graph.len() {
        for &iv in containment_graph[iu].iter() {
            for iw in containment_graph[iv].iter() {
                trimmed_containment_graph[iu].remove(iw);
            }
        }
    }
    trimmed_containment_graph
}

fn get_components(link_graph: &Vec<BTreeSet<usize>>) -> (Vec<usize>, Vec<[BTreeSet<usize>; 2]>) {
    let mut component_map: Vec<usize> = vec![0; link_graph.len()];
    let mut components: Vec<[BTreeSet<usize>; 2]> =
        vec![[BTreeSet::new(), BTreeSet::new()]; link_graph.len()];
    let mut visited: BTreeSet<usize> = BTreeSet::new();
    for s in 0..link_graph.len() {
        if visited.contains(&s) {
            continue;
        }
        let mut q: VecDeque<(usize, bool)> = VecDeque::new();
        let mut even_component: BTreeSet<usize> = BTreeSet::new();
        let mut odd_component: BTreeSet<usize> = BTreeSet::new();
        q.push_back((s, true));
        while let Some((iu, e)) = q.pop_front() {
            if visited.contains(&iu) {
                continue;
            }
            visited.insert(iu);
            if e {
                &mut even_component
            } else {
                &mut odd_component
            }
            .insert(iu);
            component_map[iu] = s;
            for &iv in link_graph[iu].iter() {
                q.push_back((iv, !e));
            }
        }
        components[s] = [even_component, odd_component];
    }
    (component_map, components)
}

fn get_lowest_solvable_depth(
    position_pairs: &Vec<(usize, usize)>,
    component_id: usize,
    maximum_height: isize,
    component_map: &Vec<usize>,
    components: &Vec<[BTreeSet<usize>; 2]>,
    containment_graph: &Vec<BTreeSet<usize>>,
    cache: &mut BTreeMap<(usize, isize), Option<isize>>,
) -> Option<isize> {
    let mut result: Option<isize>;
    if let Some(&cached_result) = cache.get(&(component_id, maximum_height)) {
        result = cached_result;
    } else if maximum_height < 0 {
        result = None;
    } else {
        result = None;
        let mut contained_components: BTreeSet<usize> = BTreeSet::new();
        for &iu in components[component_id][0]
            .iter()
            .chain(components[component_id][1].iter())
        {
            for &iv in containment_graph[iu].iter() {
                contained_components.insert(component_map[iv]);
            }
        }
        contained_components.remove(&component_id);
        for (upper_component, lower_component) in [
            (&components[component_id][0], &components[component_id][1]),
            (&components[component_id][1], &components[component_id][0]),
        ] {
            let mut current_result: Option<isize> = Some(0isize);
            for &upper_pair_id_0 in upper_component.iter() {
                let mut upper_component_height = 1isize;
                for &upper_pair_id_1 in upper_component.iter() {
                    if is_contained(
                        position_pairs[upper_pair_id_1],
                        position_pairs[upper_pair_id_0],
                    ) {
                        upper_component_height += 1;
                    }
                }
                if upper_component_height > maximum_height {
                    current_result = None;
                }
            }
            if current_result.is_none() {
                continue;
            }
            for &lower_pair_id_0 in lower_component.iter() {
                let mut lower_component_height = 1isize;
                for &lower_pair_id_1 in lower_component.iter() {
                    if is_contained(
                        position_pairs[lower_pair_id_1],
                        position_pairs[lower_pair_id_0],
                    ) {
                        lower_component_height += 1;
                    }
                }
                current_result = std::cmp::min(current_result, Some(-lower_component_height));
            }
            for &contained_component in contained_components.iter() {
                let mut next_maximum_height = maximum_height;
                for &upper_pair_id in upper_component.iter() {
                    if is_contained(
                        position_pairs[upper_pair_id],
                        position_pairs[contained_component],
                    ) {
                        next_maximum_height -= 1;
                    }
                }
                let mut depth_offset = 0isize;
                for &lower_pair_id in lower_component.iter() {
                    if is_contained(
                        position_pairs[lower_pair_id],
                        position_pairs[contained_component],
                    ) {
                        depth_offset -= 1;
                    }
                }
                current_result = std::cmp::min(
                    current_result,
                    get_lowest_solvable_depth(
                        position_pairs,
                        contained_component,
                        next_maximum_height,
                        component_map,
                        components,
                        containment_graph,
                        cache,
                    )
                    .map(|d| d + depth_offset),
                );
            }
            result = std::cmp::max(result, current_result);
        }
    }
    cache.insert((component_id, maximum_height), result);
    result
}
