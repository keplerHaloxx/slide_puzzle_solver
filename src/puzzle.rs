#![allow(unused)]

use pathfinding::prelude::*;

pub(crate) type Puzzle = Vec<Vec<u8>>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PuzzleState {
    pub(crate) puzzle: Puzzle,
    pub(crate) number_slid: u8,
}

impl PuzzleState {
    pub(crate) fn new(puzzle: Puzzle, number_slid: u8) -> Self {
        PuzzleState { puzzle, number_slid }
    }

    pub(crate) fn heuristic(&self) -> usize {
        // Manhatten distance heuristic
        let mut distance = 0;
        for i in 0..3 {
            for j in 0..3 {
                let value = self.puzzle[i][j];
                if value != 0 {
                    let target_row = (value - 1) / 3;
                    let target_col = (value - 1) % 3;
                    distance += ((i as i32) - (target_row as i32)).abs() as usize
                        + ((j as i32) - (target_col as i32)).abs() as usize;
                }
            }
        }
        distance
    }

    pub(crate) fn is_goal(&self) -> bool {
        self.puzzle == vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 0]]
    }

    pub(crate) fn successors(&self) -> Vec<(Self, usize)> {
        let mut successors = Vec::new();
        let (empty_row, empty_col) = self.find_empty();
        for &(d_row, d_col) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let new_row = empty_row.wrapping_add(d_row as usize);
            let new_col = empty_col.wrapping_add(d_col as usize);
            if new_row < 3 && new_col < 3 {
                let mut new_puzzle = self.puzzle.clone();
                new_puzzle[empty_row][empty_col] = new_puzzle[new_row][new_col];
                new_puzzle[new_row][new_col] = 0;
                let new_state = PuzzleState::new(new_puzzle, self.puzzle[new_row][new_col]);
                successors.push((new_state, 1));
            }
        }
        successors
    }

    pub(crate) fn find_empty(&self) -> (usize, usize) {
        for i in 0..3 {
            for j in 0..3 {
                if self.puzzle[i][j] == 0 {
                    return (i, j);
                }
            }
        }
        panic!("Empty cell not found!");
    }
}

pub(crate) fn solve_puzzle(initial_state: PuzzleState) -> Option<(Vec<PuzzleState>, usize)> {
    // Perform A* search
    if let Some((mut path, cost)) = astar(
        &initial_state,
        |state| state.successors(),
        |state| state.heuristic(),
        |state| state.is_goal(),
    ) {
        // Remove the initial state from the beginning of the path
        if !path.is_empty() {
            path.remove(0);
        }
        Some((path, cost))
    } else {
        None
    }
}


pub(crate) fn print_puzzle(puzzle: &Puzzle) {
    for row in puzzle {
        println!("{:?}", row);
    }
}

pub(crate) fn print_solution(path: Vec<PuzzleState>) {
    for state in path {
        println!("Slide number: {}", state.number_slid);
        print_puzzle(&state.puzzle);
        println!("---");
    }
}