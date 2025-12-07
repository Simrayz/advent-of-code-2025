use glam::IVec2;
use crate::board::Board;
use std::collections::{HashMap, HashSet};

pub fn process(input: &str) -> miette::Result<String> {
    let board = Board::from_input(input);
    let mut beams: HashSet<IVec2> = HashSet::from([board.start_position]);
    let mut visit_count: HashMap<IVec2, usize> = HashMap::from([(board.start_position, 1)]);

    for _n in 0..board.height {
        let mut new_beams: HashSet<IVec2> = HashSet::new();
        for beam in beams.iter() {
            let below = *beam + IVec2::new(0, 1);
            if board.splitter_positions.contains(&below) {
                let left = below + IVec2::new(-1, 0);
                let right = below + IVec2::new(1, 0);
                *visit_count.entry(left).or_default() += visit_count[&beam];
                *visit_count.entry(right).or_default() += visit_count[&beam];
                new_beams.insert(below + IVec2::new(-1, 0));
                new_beams.insert(below + IVec2::new(1, 0));
            } else {
                new_beams.insert(below);
                *visit_count.entry(below).or_default() += visit_count[&beam];
            }
        }
        beams = new_beams;
    }

    let last_row_visit_count: usize = visit_count
        .iter()
        .filter_map(|(pos, &count)| {
            if pos.y == board.height as i32 {
                Some(count)
            } else {
                None
            }
        })
        .sum();

    Ok(last_row_visit_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process_example() -> miette::Result<()> {
        let result = process(include_str!("../inputs/example.txt"));
        assert_eq!("40", result?);
        Ok(())
    }
}