pub fn process(input: &str) -> miette::Result<String> {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let transposed = get_transposed_matrix(&lines);

    let row_length = transposed[0].len();
    let mut total: u64 = 0;

    let mut current_operator: char = ' ';
    let mut current_numbers: Vec<u64> = Vec::new();
    for row in transposed {
        // If every character is an empty space, skip
        if row.iter().all(|&ch| ch == ' ') {
            let result = process_numbers(&current_numbers, current_operator);
            total += result;

            current_numbers.clear();
            current_operator = ' ';
            continue;
        }
        let row_str = row[..row_length - 1].iter().filter(|&&ch| ch != ' ').collect::<String>();
        let number: u64 = row_str.trim().parse().unwrap();
        current_numbers.push(number);
        if row[row_length - 1] != ' ' {
            current_operator = row[row_length - 1];
        }
    }

    if !current_numbers.is_empty() {
        let result = process_numbers(&current_numbers, current_operator);
        total += result;
    }

    Ok(total.to_string())
}

fn get_transposed_matrix(lines: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut transposed: Vec<Vec<char>> = Vec::new();
    let row_count = lines.len();
    let col_count = lines[0].len();
    for col in 0..col_count {
        let mut new_row: Vec<char> = Vec::new();
        for row in 0..row_count {
            new_row.push(lines[row][col]);
        }
        transposed.push(new_row);
    }
    transposed
}

fn process_numbers(numbers: &Vec<u64>, operator: char) -> u64 {
    match operator {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../inputs/example.txt"));
        assert_eq!("3263827", result?);
        Ok(())
    }
}
