use crate::board::Board;

pub fn process(input: &str) -> miette::Result<String> {
    let board = Board::from_input(input);
    
    let mut total = 0;
    for roll in &board.initial {
        let neighbor_count = board.neighbor_count(*roll);
        if neighbor_count < 4 {
            total += 1;
        }
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../inputs/example.txt"));
        assert_eq!("13", result?);
        Ok(())
    }
}
