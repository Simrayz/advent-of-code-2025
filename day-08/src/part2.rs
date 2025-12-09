use glam::IVec3;
use itertools::Itertools;

pub fn process(input: &str) -> miette::Result<String> {
    let junction_count = input.lines().count();
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

    let mut circuits: Vec<Vec<IVec3>> = Vec::new();
    let mut last_pair: Vec<IVec3> = Vec::new();

    for (a, b, _) in points {
        let from_circuit_idx = circuits
            .iter()
            .position(|circuit| circuit.contains(&a));
        let to_circuit_idx = circuits
            .iter()
            .position(|circuit| circuit.contains(&b));

        match (from_circuit_idx, to_circuit_idx) {
            (Some(from_idx), Some(to_idx)) if from_idx != to_idx => {
                let circuit_from = circuits.remove(from_idx.max(to_idx));
                let circuit_to = circuits.remove(from_idx.min(to_idx));
                let merged: Vec<_> = circuit_from.iter()
                    .chain(circuit_to.iter())
                    .cloned()
                    .collect();
                circuits.push(merged);
            }
            (Some(_), Some(_)) => {}
            (Some(idx), None) => {
                circuits[idx].push(b);
            }
            (None, Some(idx)) => {
                circuits[idx].push(a);
            }
            (None, None) => {
                circuits.push(vec![a, b]);
            }
        }

        if circuits[0].len() == junction_count {
            last_pair = vec![a, b];
            break;
        }
    }

    let last_product = (last_pair[0].x as i64) * (last_pair[1].x as i64);

    Ok(last_product.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../inputs/example.txt"));
        assert_eq!("25272", result?);
        Ok(())
    }
}
