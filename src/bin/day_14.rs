use advent_of_code_2023_in_rust::grid::parse_char_grid;
use ndarray::{s, Array2};
use std::collections::HashMap;

enum Direction {
    North,
    South,
    West,
    East,
}

fn main() {
    // Parse the input, find the start cell and calculate the distances from it
    let input = include_str!("../../puzzle_inputs/day_14.txt");
    let grid = parse_char_grid(input, |c| c);

    // Solve 14
    let sol_14: usize = solve_14a(grid.clone());
    let correct_sol_14: usize = 106990;
    println!("* 14 *");
    println!("My solution: {sol_14}");
    println!("Correct solution: {correct_sol_14}");
    println!("Equal: {}\n", sol_14 == correct_sol_14);

    // Solve 14b
    let sol_14b: usize = solve_14b(grid.clone());
    let correct_sol_14b: usize = 100531;
    println!("* 14b *");
    println!("My solution: {sol_14b}");
    println!("Correct solution: {correct_sol_14b}");
    println!("Equal: {}\n", sol_14b == correct_sol_14b);
}

fn solve_14a(grid: Array2<char>) -> usize {
    score(&tilt(grid, Direction::North))
}

fn solve_14b(mut grid: Array2<char>) -> usize {
    // Find a cycle in the grid transitions
    let mut found_grids: HashMap<Array2<_>, usize> = HashMap::from([(grid.clone(), 0)]);
    let cycle_idx = {
        let mut i = 1;
        loop {
            grid = cycle(grid);
            if let Some(cycle_idx) = found_grids.get(&grid) {
                break *cycle_idx;
            }
            found_grids.insert(grid.clone(), i);
            i += 1;
        }
    };

    // Now find the 1000000000th grid
    let nth: usize = 1000000000;
    let idx = nth
        .checked_sub(cycle_idx)
        .map(|n| {
            let cycle_len = found_grids.len() - cycle_idx;
            let n = n % cycle_len;
            cycle_idx + n
        })
        .unwrap_or(nth);
    let grid = found_grids.iter().find(|(_, &i)| i == idx).unwrap().0;

    // Score the state
    score(grid)
}

fn tilt(mut grid: Array2<char>, dir: Direction) -> Array2<char> {
    let axis = match dir {
        Direction::North | Direction::South => ndarray::Axis(1),
        Direction::West | Direction::East => ndarray::Axis(0),
    };
    for mut col in grid.axis_iter_mut(axis) {
        let mut col = match dir {
            Direction::North | Direction::West => col.slice_mut(s![..;1]),
            Direction::South | Direction::East => col.slice_mut(s![..;-1]),
        };
        let (mut idx_to, mut idx_from) = (0, 1);
        while idx_from < col.len() && idx_to < col.len() {
            if idx_to >= idx_from {
                idx_from = idx_to + 1;
            } else {
                match (col[idx_from], col[idx_to]) {
                    (_, 'O') | (_, '#') => idx_to += 1,
                    ('O', '.') => col.swap(idx_from, idx_to),
                    ('#', _) => (idx_to, idx_from) = (idx_from + 1, idx_from + 2),
                    (_, '.') => idx_from += 1,
                    (_, _) => unreachable!(),
                }
            }
        }
    }
    grid
}

#[allow(clippy::let_and_return)]
fn cycle(grid: Array2<char>) -> Array2<char> {
    let grid = tilt(grid, Direction::North);
    let grid = tilt(grid, Direction::West);
    let grid = tilt(grid, Direction::South);
    let grid = tilt(grid, Direction::East);
    grid
}

fn score(grid: &Array2<char>) -> usize {
    grid.indexed_iter()
        .filter_map(|((i, _), &c)| match c {
            'O' => Some(grid.shape()[0] - i),
            _ => None,
        })
        .sum()
}
