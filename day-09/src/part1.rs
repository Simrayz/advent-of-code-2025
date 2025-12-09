use std::collections::{HashSet};

pub fn process(input: &str) -> miette::Result<String> {
    let points = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|n| n.parse::<i32>().unwrap());
            (coords.next().unwrap(), coords.next().unwrap())
        })
        .collect::<Vec<(i32, i32)>>();

    let mut seen_points: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    let mut max_area = 0;
    let half_point = points.len() / 2;
    for point in points[..half_point].iter() {
        for other_point in points[half_point..].iter() {
            if point == other_point {
                continue;
            }
            if seen_points.contains(&(*point, *other_point)) {
                continue;
            }
            let area = get_square(*point, *other_point);
            if area > max_area {
                max_area = area;
            }
            seen_points.insert((*point, *other_point));
        }
    }

    Ok(max_area.to_string())
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
        assert_eq!("50", result?);
        Ok(())
    }
}
