
#[derive(Debug)]
pub enum Direction {
  Left(i32),
  Right(i32),
}

impl Direction {
  pub fn from_input(input: &str) -> Vec<Direction> {
    input
      .lines()
      .map(|line| {
        let (direction, number) = line.split_at(1);
        let value: i32 = number.parse().unwrap();
        match direction {
          "L" => Direction::Left(value),
          "R" => Direction::Right(value),
          _ => panic!("Invalid direction"),
        }
      })
      .collect()
  }
}