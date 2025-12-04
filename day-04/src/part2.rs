use std::collections::HashSet;
use crate::board::Board;

pub fn process(input: &str) -> miette::Result<String> {
    let mut board = Board::from_input(input);
    
    loop {
        let should_continue = remove_rolls(&mut board);
        if !should_continue {
            break;
        }
    }

    let removed_count = board.initial.len() - board.rolls.len();

    Ok(removed_count.to_string())
}

fn remove_rolls(board: &mut Board) -> bool {
    let mut new_rolls = HashSet::new();

    for &roll in board.rolls.iter() {
        let neighbor_count = board.neighbor_count(roll);
        if neighbor_count < 4 {
            continue;
        }
        new_rolls.insert(roll);
    }
    if new_rolls.len() < board.rolls.len() {
        board.rolls = new_rolls.clone();
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../inputs/example.txt"));
        assert_eq!("43", result?);
        Ok(())
    }
}
