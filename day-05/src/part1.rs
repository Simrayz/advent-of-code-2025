use std::ops::RangeInclusive;

pub fn process(input: &str) -> miette::Result<String> {
    let mut ranges: Vec<RangeInclusive<i64>> = Vec::new();
    let mut numbers: Vec<i64> = Vec::new();

    let mut has_parsed_ranges = false;
    for line in input.lines() {
        if line.trim() == "" {
            has_parsed_ranges = true;
            continue;
        }
        if !has_parsed_ranges {
            let parts: Vec<&str> = line.trim().split('-').collect();
            let start: i64 = parts[0].parse().unwrap();
            let end: i64 = parts[1].parse().unwrap();
            ranges.push(RangeInclusive::new(start, end));
        } else {
            let number: i64 = line.trim().parse().unwrap();
            numbers.push(number);
        }
    }

    let count = numbers.iter().filter(|&&num| {
        for range in &ranges {
            if range.contains(&num) {
                return true;
            }
        }
        return false;
    }).count();

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
