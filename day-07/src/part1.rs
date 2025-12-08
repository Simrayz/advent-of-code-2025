use std::collections::HashSet;

use glam::IVec2;

use crate::board::Board;

pub fn process(input: &str) -> miette::Result<String> {
    let board = Board::from_input(input);
    let mut beams: HashSet<IVec2> = HashSet::from([board.start_position]);

    let mut split_count = 0;

    for _n in 0..board.height {
        let mut new_beams: HashSet<IVec2> = HashSet::new();
        for beam in beams.iter() {
            let below = *beam + IVec2::new(0, 1);
            if board.splitter_positions.contains(&below) {
                new_beams.insert(below + IVec2::new(-1, 0));
                new_beams.insert(below + IVec2::new(1, 0));
                split_count += 1;
            } else {
                new_beams.insert(below);
            }
        }
        beams = new_beams;
    }

    Ok(split_count.to_string())
}

pub fn process_fast(input: &str) -> miette::Result<String> {
    let columns = input.lines().next().unwrap().chars().count();
    let mut counter = vec![0; columns];
    let mut total_splits = 0;

    input.lines().step_by(2).for_each(|line| {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    counter[x] += 1;
                },
                '^' => {
                    let count = counter[x];
                    counter[x] = 0;
                    counter[x - 1] = 1;
                    counter[x + 1] = 1;
                    if count > 0 {
                        total_splits += 1;
                    }
                },
                _ => {}
            }
        }
        // println!("{:?}", counter);
    });
    Ok(total_splits.to_string())
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
