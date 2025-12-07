use std::collections::HashSet;

use glam::IVec2;

use crate::board::Board;

pub fn process(input: &str) -> miette::Result<String> {
    let board = Board::from_input(input);

    let mut beams = HashSet::from([board.start_position]);
    let mut split_count = 0;

    for _n in 0..board.height {
        let mut new_beams: HashSet<IVec2> = HashSet::new();
        let mut x_positions: HashSet<i32> = HashSet::new();
        for beam in beams.iter() {
            let below = *beam + IVec2::new(0, 1);
            if board.splitter_positions.contains(&below) {
                // Only count split if neither new beam's X is already present
                let left = below + IVec2::new(-1, 0);
                let right = below + IVec2::new(1, 0);
                if !x_positions.contains(&left.x) {
                    new_beams.insert(left);
                    x_positions.insert(left.x);
                }
                if !x_positions.contains(&right.x) {
                    new_beams.insert(right);
                    x_positions.insert(right.x);
                }
                split_count += 1;
            } else {
                if !x_positions.contains(&below.x) {
                    new_beams.insert(below);
                    x_positions.insert(below.x);
                }
            }
        }
        beams = new_beams;
    }

    Ok(split_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../inputs/example.txt"));
        assert_eq!("21", result?);
        Ok(())
    }
}
