
/*
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
*/

use std::{collections::{HashSet, VecDeque}, vec};

#[derive(Debug)]
pub struct Machine {
    pub goal: u8,
    pub goal_parts: Vec<u8>,
    pub operations: Vec<u8>,
    pub size: usize,
}

impl Machine {
    pub fn new(input_line: &str) -> Self {
        let parts = input_line.split_whitespace().collect::<Vec<&str>>();
        let goal_parts = parts[0]
            .trim_matches(&['[', ']'] as &[_])
            .chars()
            .filter_map(|c| match c {
                '#' => Some(1_u8),
                '.' => Some(0_u8),
                _ => None,
            })
            .collect::<Vec<u8>>();
        let goal = goal_parts.iter().fold(0_u8, |acc, &bit| (acc << 1) | bit);

        let size = goal_parts.len();
        let operation_idxs: Vec<Vec<u8>> = parts[1..parts.len() - 1].iter().map(|button|{
            button
                .trim_matches(&['(', ')'] as &[_])
                .split(',')
                .filter_map(|s| s.parse::<u8>().ok())
                .collect::<Vec<u8>>()
        }).collect();
        let operations = operation_idxs.iter().map(|idxs| {
            let mut op_mask = vec![0_u8; size];
            for &idx in idxs {
                op_mask[idx as usize] = 1;
            }
            // Convert the binary mask to a u8 number
            op_mask.iter().fold(0_u8, |acc, &bit| (acc << 1) | bit)
        }).collect::<Vec<u8>>();

        Machine {
            goal,
            goal_parts,
            operations,
            size
        }
    }
    pub fn solve(&self) -> Option<usize> {
        let state = 0_u8;
        let mut queue = VecDeque::from([(state, 0)]);
        let mut visited = HashSet::with_capacity(1 << self.size);
        visited.insert(state);

        while let Some((current_state, steps)) = queue.pop_front() {
            if current_state == self.goal {
                return Some(steps);
            }
            for &operation in &self.operations {
                let next_state = current_state ^ operation;
                if visited.insert(next_state) {
                    queue.push_back((next_state, steps + 1));
                }
            }
        }
        None
    }
}