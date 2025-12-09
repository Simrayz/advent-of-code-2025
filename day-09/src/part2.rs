use itertools::Itertools;

pub fn process(input: &str) -> miette::Result<String> {
    let points = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|n| n.parse::<i32>().unwrap());
            (coords.next().unwrap(), coords.next().unwrap())
        })
        .collect::<Vec<(i32, i32)>>();

    let edges: Vec<((i32, i32), (i32, i32))> = points
        .iter()
        .circular_tuple_windows()
        .map(|(&from, &to)| (from, to))
        .collect();

    let max_box = points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let area = get_square(*a, *b);
            (a, b, area)
        })
        .sorted_by_key(|v| v.2)
        .rev()
        .find(|(a, b, _)| {
            edges.iter().all(|(e_from, e_to)| {
                let to_left = a.0.max(b.0) <= e_from.0.min(e_to.0);
                let to_right = a.0.min(b.0) >= e_from.0.max(e_to.0);
                let above = a.1.max(b.1) <= e_from.1.min(e_to.1);
                let below = a.1.min(b.1) >= e_from.1.max(e_to.1);
                to_left || to_right || above || below
            })
        });

    Ok(max_box.unwrap().2.to_string())
}

fn get_square(from: (i32, i32), to: (i32, i32)) -> i64 {
    let width = (to.0 - from.0).abs() as i64 + 1;
    let height = (to.1 - from.1).abs() as i64 + 1;
    return width * height;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let result = process(include_str!("../inputs/example.txt"));
        assert_eq!("24", result?);
        Ok(())
    }
}
