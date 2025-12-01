pub fn process(input: &str) -> miette::Result<String> {
    let moves: Vec<i32> = input
        .lines()
        .map(|line| {
            let (direction, number) = line.split_at(1);
            let value: i32 = number.parse().unwrap();
            match direction {
                "L" => -value,
                "R" => value,
                _ => 0,
            }
        })
        .collect();
    
    let mut count = 0;
    let mut dial = 50;

    for mv in moves {
        if mv > 0 {
            let passes = (dial + mv).div_euclid(100);
            dial = (dial + mv).rem_euclid(100);
            count += passes;
        } else {
            let flipped_dial = (100 - dial).rem_euclid(100);
            let passes = (flipped_dial - mv).div_euclid(100);
            count += passes;
            dial = (dial + mv).rem_euclid(100);
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
