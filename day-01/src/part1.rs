use crate::direction::Direction;

pub fn process(input: &str) -> miette::Result<String> {
    let direction_moves = Direction::from_input(input);

    let mut count = 0;
    let mut dial = 50;

    for mv in direction_moves {
        match mv {
            Direction::Left(v) => {
                dial = (dial - v).rem_euclid(100);
            }
            Direction::Right(v) => {
                dial = (dial + v).rem_euclid(100);
            }
        }
        if dial == 0 {
            count += 1;
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
        assert_eq!("3", result?);
        Ok(())
    }
}
