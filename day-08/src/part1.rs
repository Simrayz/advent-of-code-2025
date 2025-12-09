use std::collections::{HashSet};

use glam::IVec3;
use itertools::Itertools;

pub fn process(input: &str, max_connections: usize) -> miette::Result<String> {
    let points: Vec<(IVec3, IVec3, i32)> = input
        .lines()
        .map(|line| {
            let coords: Vec<i32> = line
                .split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            IVec3::new(coords[0], coords[1], coords[2])
        })
        .tuple_combinations()
        .map(|(a, b)| {
            (a, b, a.as_vec3().distance(b.as_vec3()) as i32)
        })
        .sorted_by(|a, b| a.2.cmp(&b.2))
        .collect();

    let mut circuits: Vec<HashSet<IVec3>> = Vec::new();

    for (a, b, _) in points.iter().take(max_connections) {
        let from_circuit_idx = circuits
            .iter()
            .position(|circuit| circuit.contains(a));
        let to_circuit_idx = circuits
            .iter()
            .position(|circuit| circuit.contains(b));

        match (from_circuit_idx, to_circuit_idx) {
            (Some(from_idx), Some(to_idx)) if from_idx != to_idx => {
                let circuit_from = circuits.remove(from_idx.max(to_idx));
                let circuit_to = circuits.remove(from_idx.min(to_idx));
                let merged: HashSet<IVec3> = circuit_from
                    .into_iter()
                    .chain(circuit_to)
                    .collect();
                circuits.push(merged);
            }
            (Some(_), Some(_)) => {}
            (Some(idx), None) => {
                circuits[idx].insert(*b);
            }
            (None, Some(idx)) => {
                circuits[idx].insert(*a);
            }
            // Neither point in any circuit - create new
            (None, None) => {
                circuits.push(HashSet::from([*a, *b]));
            }
        }
    }

    let circuit_sizes = circuits.iter().map(|c| c.len()).collect::<Vec<usize>>();
    let circuit_product: usize = circuit_sizes
        .iter()
        .cloned()
        .unique()
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .product();

    Ok(circuit_product.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../inputs/example.txt"), 10);
        assert_eq!("40", result?);
        Ok(())
    }
}
