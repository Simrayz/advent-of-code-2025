use std::collections::HashSet;

use glam::IVec2;

const DIRECTIONS: [[IVec2; 3]; 8] = [
    [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)], // east
    [IVec2::new(0, -1), IVec2::new(0, -2), IVec2::new(0, -3)], // west
    [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)], // south east
    [IVec2::new(1, -1), IVec2::new(2, -2), IVec2::new(3, -3)], // north east
    [IVec2::new(-1, 1), IVec2::new(-2, 2), IVec2::new(-3, 3)], // south west
    [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3)], // north west
    [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)], // south
    [IVec2::new(-1, 0), IVec2::new(-2, 0), IVec2::new(-3, 0)], // north
];

#[derive(Debug)]
pub struct Board {
    pub width: i32,
    pub height: i32,
    pub initial: HashSet<IVec2>,
    pub rolls: HashSet<IVec2>, // Mutable field to track current rolls
}

impl Board {
    pub fn from_input(input: &str) -> Self {
        build_board(input)
    }
    pub fn neighbor_count(&self, position: IVec2) -> usize {
        let neighbors: Vec<IVec2> = DIRECTIONS.iter().filter_map(|direction| {
            let check = position + direction[0];
            if self.rolls.contains(&check) {
                Some(check)
            } else {
                None
            }
        }).collect();
        
        return neighbors.len();
    }
    pub fn print_board(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = IVec2::new(x, y);
                if self.rolls.contains(&pos) {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}



fn build_board(input: &str) -> Board {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut visited: HashSet<IVec2> = std::collections::HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            
            match ch {
                '@' => {
                    visited.insert(IVec2::new(x as i32, y as i32));
                }
                _ => {}
            }
        }
    }

    // Return board
    return Board {
        width,
        height,
        initial: visited.clone(),
        rolls: visited,
    };
}