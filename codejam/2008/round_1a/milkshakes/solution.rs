use std::collections::HashSet;

struct CustomerPreference {
    malted: Option<usize>,
    unmalted: HashSet<usize>,
}

fn adjust_customer_preferences(
    c: usize,
    malted_flavors: &mut Vec<bool>,
    customer_preferences: &mut Vec<CustomerPreference>,
    customers_for_unmalted_flavors: &Vec<HashSet<usize>>,
) -> bool {
    if customer_preferences[c].unmalted.len() > 0 {
        return true;
    }
    if let Some(f) = customer_preferences[c].malted {
        if malted_flavors[f] {
            return true;
        }
        malted_flavors[f] = true;
        for &nc in customers_for_unmalted_flavors[f].iter() {
            customer_preferences[nc].unmalted.remove(&f);
        }
        for &nc in customers_for_unmalted_flavors[f].iter() {
            if !adjust_customer_preferences(
                nc,
                malted_flavors,
                customer_preferences,
                customers_for_unmalted_flavors,
            ) {
                return false;
            }
        }
        return true;
    }
    return false;
}

fn minimum_malted_flavors(
    n: usize,
    mut customer_preferences: Vec<CustomerPreference>,
) -> Option<Vec<bool>> {
    let mut malted_flavors = vec![false; n];

    let mut customers_for_unmalted_flavors = vec![HashSet::new(); n];
    for (c, customer_preference) in customer_preferences.iter().enumerate() {
        for &unmalted_preference in customer_preference.unmalted.iter() {
            customers_for_unmalted_flavors[unmalted_preference].insert(c);
        }
    }

    for c in 0..customer_preferences.len() {
        if !adjust_customer_preferences(
            c,
            &mut malted_flavors,
            &mut customer_preferences,
            &customers_for_unmalted_flavors,
        ) {
            return None;
        }
    }

    Some(malted_flavors)
}

fn main() {
    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer).unwrap();
    let test_case_count = buffer.trim().parse::<usize>().unwrap();
    buffer.clear();

    for test_case_index in 1..1 + test_case_count {
        std::io::stdin().read_line(&mut buffer).unwrap();
        let n = buffer.trim().parse::<usize>().unwrap();
        buffer.clear();

        std::io::stdin().read_line(&mut buffer).unwrap();
        let m = buffer.trim().parse::<usize>().unwrap();
        buffer.clear();

        let mut customer_preferences: Vec<CustomerPreference> = Vec::new();

        for _ in 0..m {
            std::io::stdin().read_line(&mut buffer).unwrap();
            let preferences = buffer
                .split(" ")
                .into_iter()
                .map(|w| w.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            buffer.clear();

            let mut malted_preference: Option<usize> = None;
            let mut unmalted_preferences: HashSet<usize> = HashSet::new();

            for i in (1..preferences.len()).step_by(2) {
                if preferences[i + 1] == 0 {
                    unmalted_preferences.insert(preferences[i] - 1);
                } else {
                    malted_preference = Some(preferences[i] - 1);
                }
            }

            customer_preferences.push(CustomerPreference {
                malted: malted_preference,
                unmalted: unmalted_preferences,
            });
        }

        println!(
            "Case #{test_case_index}: {}",
            if let Some(malted_flavors) = minimum_malted_flavors(n, customer_preferences) {
                malted_flavors
                    .into_iter()
                    .map(|x| if x { "1".to_string() } else { "0".to_string() })
                    .collect::<Vec<String>>()
                    .join(" ")
            } else {
                "IMPOSSIBLE".to_string()
            }
        );
    }
}
