use miette::Result;

pub fn process(input: &str) -> Result<String> {
    let id_ranges = build_ranges(input);

    let mut total: u64 = 0;
    for ids in id_ranges.into_iter() {
        for id in ids.into_iter() {
            let id_str = id.to_string();
            let half = id_str.len() / 2;

            for limit in 1..=half {
                if is_repeated(&id_str, limit) {
                    total += id;
                    break;
                }
            }
    }
}

    Ok(total.to_string())
}

fn is_repeated(s: &str, len: usize) -> bool {
    if s.len() % len != 0 {
        return false;
    }
    let pattern = &s[..len];
    pattern.repeat(s.len() / len) == s
}

fn build_ranges(input: &str) -> Vec<std::ops::RangeInclusive<u64>> {
    input
        .trim()
        .split(",")
        .filter_map(|pair| {
            let parts: Vec<_> = pair.split('-').map(|s| s.parse::<u64>().ok()).collect();
            if parts.len() == 2 && parts[0].is_some() && parts[1].is_some() {
                Some(parts[0].unwrap()..=parts[1].unwrap())
            } else {
                None
            }
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> Result<()> {
        let result = process(include_str!("../inputs/example.txt"));
        assert_eq!("4174379265", result?);
        Ok(())
    }
}
