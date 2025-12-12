pub fn process(input: &str) -> miette::Result<String> {
    let cleaned_input = input.replace("\r\n", "\n");
    let chunks = cleaned_input.split("\n\n").collect::<Vec<&str>>();

    let region_chunk = chunks[chunks.len() - 1];

    let regions: Vec<(usize, usize)> = region_chunk
        .lines()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let area = parts[0]
                .split("x")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let required_packages = parts[1]
                .split(" ")
                .map(|s| s.parse::<usize>().unwrap())
                .sum();
            (area[0] * area[1], required_packages)
        })
        .collect();

    let valid_regions = regions.iter().fold(0, |acc, region| {
        let (region_area, required_packages) = region;
        dbg!(region_area, required_packages);

        if *region_area <= required_packages * 9 {
            acc + 1
        } else {
            acc
        }
    });

    Ok(valid_regions.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../inputs/example.txt"));
        assert_eq!("2", result?);
        Ok(())
    }
}
