use std::collections::HashSet;

use glam::IVec2;

pub struct Board {
  pub start_position: glam::IVec2,
  pub splitter_positions: HashSet<glam::IVec2>,
  pub width: usize,
  pub height: usize,
}

impl Board {
  pub fn from_input(input: &str) -> Self {
    let (start_position, splitter_positions) = Board::build_positions(input);
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    Board {
      start_position,
      splitter_positions,
      width,
      height,
    }
  }
  
  fn build_positions(input: &str) -> (IVec2, HashSet<IVec2>) {
    let mut start_position: IVec2 = IVec2::ZERO;
    let mut splitter_positions: HashSet<IVec2> = HashSet::new();

    for (line_index, line) in input.lines().enumerate() {
      for (char_index, ch) in line.chars().enumerate() {
        match ch {
          'S' => {
            start_position = IVec2::new(char_index as i32, line_index as i32);
          }
          '^' => {
            splitter_positions.insert(IVec2::new(char_index as i32, line_index as i32));
          }
          _ => {}
        }
      }
    }

    return (start_position, splitter_positions);
  }
  
}