pub fn process(input: &str) -> miette::Result<String> {
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for line in input.lines() {
        if line.trim() == "" {
            break;
        }
        let parts: Vec<&str> = line.trim().split('-').collect();
        let start: i64 = parts[0].parse().unwrap();
        let end: i64 = parts[1].parse().unwrap();
        ranges.push((start, end));
    }

    // Sort ranges by start position
    ranges.sort_by_key(|r| r.0);

    // Merge overlapping ranges and calculate total coverage
    let mut merged_ranges: Vec<(i64, i64)> = Vec::new();
    
    for (start, end) in ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if start <= last.1 + 1 {
                // Overlapping or adjacent - extend the last range
                last.1 = last.1.max(end);
            } else {
                // Non-overlapping - add new range
                merged_ranges.push((start, end));
            }
        } else {
            merged_ranges.push((start, end));
        }
    }

    let total: i64 = merged_ranges.iter()
        .map(|(start, end)| end - start + 1)
        .sum();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../inputs/example.txt"));
        assert_eq!("14", result?);
        Ok(())
    }
}
