use crate::machine::Machine;

pub fn process(input: &str) -> miette::Result<String> {
    let solution: usize = input
        .lines()
        .filter_map(|line| Machine::new(line).solve())
        .sum();
    Ok(solution.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../inputs/example.txt"));
        assert_eq!("7", result?);
        Ok(())
    }
}
