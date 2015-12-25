fn calculate_total_weight(packages: &[usize]) -> usize {
    packages.iter().fold(0, |a, b| a + b)
}

fn calculate_quantum_entanglement(packages: &[usize]) -> usize {
    packages.iter().fold(1, |a, b| a * b)
}

fn get_groups_for_weight_and_max_size(packages: &[usize],
                                      weight: usize,
                                      max_size: usize)
                                      -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    if max_size > 0 && weight > 0 && packages.len() > 0 {
        let first_package = packages[0];
        let inner_groups = get_groups_for_weight_and_max_size(&packages[1..], weight, max_size);
        result.extend(inner_groups);

        if first_package <= weight {
            let inner_groups = get_groups_for_weight_and_max_size(&packages[1..],
                                                                  weight - first_package,
                                                                  max_size - 1);

            for inner_group in inner_groups {
                let mut group = Vec::new();
                group.push(first_package);
                group.extend(inner_group);
                result.push(group);
            }
        }
    } else if weight == 0 {
        result.push(Vec::new());
    }
    result
}

fn find_min_quantum_entanglement(packages: &[usize], group_count: usize) -> Option<usize> {
    let weight_per_group = calculate_total_weight(&packages) / group_count;
    for group_size in 1..packages.len() + 1 {
        let group_possibilities = get_groups_for_weight_and_max_size(&packages,
                                                                     weight_per_group,
                                                                     group_size);
        if group_possibilities.len() > 0 {
            return group_possibilities.into_iter()
                                      .map(|a| calculate_quantum_entanglement(&a))
                                      .min();
        }
    }

    None
}

fn main() {
    let mut packages = [1, 3, 5, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 67, 71, 73,
                        79, 83, 89, 97, 101, 103, 107, 109, 113];
    packages.sort_by(|a, b| b.cmp(a));

    println!("Minimum Quantum Entanglement (3): {}",
             find_min_quantum_entanglement(&packages, 3)
                 .map(|i| i.to_string())
                 .unwrap_or_else(|| "-".to_string()));

    println!("Minimum Quantum Entanglement (4): {}",
             find_min_quantum_entanglement(&packages, 4)
                 .map(|i| i.to_string())
                 .unwrap_or_else(|| "-".to_string()));
}
