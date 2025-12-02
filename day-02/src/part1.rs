use miette::{Result};

pub fn process(input: &str) -> Result<String> {
    let id_ranges = build_ranges(input);
    
    let mut total: u64 = 0;
    for ids in id_ranges.into_iter() {
        for id in ids.into_iter() {
            let id_str = id.to_string();
            let half = id_str.len() / 2;
            if &id_str[..half] == &id_str[half..] {
                total += id;
            }
        }
    }

    Ok(total.to_string())
}

fn build_ranges(input: &str) -> Vec<std::ops::RangeInclusive<u64>> {
    input
        .trim()
        .split(",")
        .map(|pair| {
            let parts: Vec<u64> = pair.split('-').map(|s| s.parse().unwrap()).collect();
            parts[0]..=parts[1]
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> Result<()> {
        let result = process(include_str!("../inputs/example.txt"));
        assert_eq!("1227775554", result?);
        Ok(())
    }
}
