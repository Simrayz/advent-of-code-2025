pub fn process(input: &str) -> miette::Result<String> {
    let lines = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let problem_count = lines[0].len();
    let mut total: u64 = 0;
    for col in 0..problem_count {
        let mut numbers: Vec<u64> = Vec::new();
        let operator = lines[lines.len()-1][col];
        for row in 0..lines.len()-1 {
            let num = lines[row][col].parse::<u64>().unwrap();
            numbers.push(num);
        }
        match operator {
            "+" => {
                let sum: u64 = numbers.iter().sum();
                total += sum;
            }
            "*" => {
                let product: u64 = numbers.iter().product();
                total += product;
            }
            _ => {
                println!("Unknown operator '{}' for column {}", operator, col);
            }
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
        assert_eq!("4277556", result?);
        Ok(())
    }
}
