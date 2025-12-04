pub fn process(input: &str) -> miette::Result<String> {
    let total = input
        .lines()
        // Split string of characters into vector of u8
        .map(|line| {
            // Split each line into individual characters and parse to u32
            let nums = line.chars()
                .map(|num| num.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let largest_indices = find_largest_number_indices(&nums[..nums.len() - 1]);
            let mut largest_nums = Vec::new();
            for idx in largest_indices {
                let second_number = find_largest_number(&nums[idx+1..]);
                largest_nums.push(nums[idx] * 10 + second_number);
            }

            largest_nums.into_iter().max().unwrap()
        })
        .sum::<u32>();

    Ok(total.to_string())
}

fn find_largest_number(numbers: &[u32]) -> u32 {
    let mut largest_value: u32 = 0;

    for &value in numbers.iter() {
        if value > largest_value {
            largest_value = value;
        }
    }

    largest_value
}

fn find_largest_number_indices(numbers: &[u32]) -> Vec<usize> {
    let mut largest_indices: Vec<usize> = Vec::new();
    let mut largest_value: u32 = 0;

    for (index, &value) in numbers.iter().enumerate() {
        if value > largest_value {
            largest_value = value;
            largest_indices.clear();
            largest_indices.push(index);
        } else if value == largest_value {
            largest_indices.push(index);
        }
    }

    largest_indices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../inputs/example.txt"));
        assert_eq!("357", result?);
        Ok(())
    }
}
