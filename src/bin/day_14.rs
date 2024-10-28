use advent_of_code_2023_in_rust::grid::parse_char_grid;
use ndarray::{s, Array2};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
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
    let mut found_grids: HashMap<Array2<_>, usize> = HashMap::from([(grid.clone(), 0)]);

    let cycle_index = {
        let mut i = 1;
        loop {
            grid = cycle(grid);
            if let Some(cycle_index) = found_grids.get(&grid) {
                break *cycle_index;
            }
            found_grids.insert(grid.clone(), i);
            i += 1;
        }
    };
    let the_nth = 1000000000;
    score(nth(the_nth, &found_grids, cycle_index))
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
        //println!("col BEF: {}", col.iter().collect::<String>());
        let mut idx_to = 0;
        let mut idx_from = 1;
        loop {
            //println!("col ---: {}", col.iter().collect::<String>());
            //println!(
            //    "       : {}",
            //    (0..col.len())
            //        .map(|i| {
            //            if idx_from == i {
            //                if idx_to == i {
            //                    '*'
            //                } else {
            //                    'f'
            //                }
            //            } else if idx_to == i {
            //                't'
            //            } else {
            //                ' '
            //            }
            //        })
            //        .collect::<String>()
            //);
            //println!();
            match (idx_from, idx_to, col.get(idx_from), col.get(idx_to)) {
                // Break if idx_from is out of bounds
                (idx_from, _, _, _) if (idx_from >= col.len()) => break,

                // Break if idx_to is out of bounds
                (_, idx_to, _, _) if (idx_to >= col.len()) => break,

                // Increment idx_to if it points to a rock
                (_, _, _, Some('O')) | (_, _, _, Some('#')) => idx_to += 1,

                // Ensure that idx_to is less than idx_from
                (_, _, _, _) if (idx_to >= idx_from) => idx_from = idx_to + 1,

                // Shift the rock to the north if possible
                (_, _, Some('O'), Some('.')) => col.swap(idx_from, idx_to),

                (_, _, Some('#'), _) => {
                    idx_to = idx_from + 1;
                    idx_from = idx_to + 1;
                }
                (_, _, _, Some('.')) => idx_from += 1,

                (idx_from, idx_to, elt_from, elt_to) => unreachable!(
                    "impossible: ({}, {}, {:?}, {:?})",
                    idx_from, idx_to, elt_from, elt_to
                ),
            }
        }
        //println!("col AFT: {}", col.iter().collect::<String>());
        //println!();
    }
    grid
}

fn nth(n: usize, found_grids: &HashMap<Array2<char>, usize>, cycle_idx: usize) -> &Array2<char> {
    let idx = n
        .checked_sub(cycle_idx)
        .map(|n| {
            let cycle_len = found_grids.len() - cycle_idx;
            let n = n % cycle_len;
            cycle_idx + n
        })
        .unwrap_or(n);
    found_grids.iter().find(|(_, &i)| i == idx).unwrap().0
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
