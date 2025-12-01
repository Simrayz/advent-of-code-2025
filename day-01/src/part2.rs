use crate::direction::Direction;

pub fn process(input: &str) -> miette::Result<String> {
    let direction_moves = Direction::from_input(input);
    
    let mut count = 0;
    let mut dial = 50;

    for mv in direction_moves {
        match mv {
            Direction::Left(v) => {
                // Convert left move to equivalent right move for counting passes
                let flipped_dial = (100_i32 - dial).rem_euclid(100);
                let passes = (flipped_dial + v).div_euclid(100);
                count += passes;
                dial = (dial - v).rem_euclid(100);
            }
            Direction::Right(v) => {
                let passes = (dial + v).div_euclid(100);
                dial = (dial + v).rem_euclid(100);
                count += passes;
            }
        }
    }

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../inputs/example.txt"));
        assert_eq!("6", result?);
        Ok(())
    }
}
