use advent_of_code_2023_in_rust::grid::parse_char_grid;
use itertools::Itertools;
use ndarray::{Array2, Axis};

fn main() {
    // Parse input and figure out how much the grid expands in each direction
    let input = include_str!("../../puzzle_inputs/day_11.txt");
    let grid = parse_char_grid(input, |c| c == '#');
    let expansion_x = expansion(&grid, 1);
    let expansion_y = expansion(&grid, 0);

    // Solve 11a
    let sol_11a = solve(&grid, &expansion_x, &expansion_y, 2);
    let correct_sol_11a: usize = 10033566;
    println!("* 11a *");
    println!("My solution: {sol_11a}");
    println!("Correct solution: {correct_sol_11a}");
    println!("Equal: {}\n", sol_11a == correct_sol_11a);

    // Solve 11b
    let sol_11b = solve(&grid, &expansion_x, &expansion_y, 1000000);
    let correct_sol_11b: usize = 560822911938;
    println!("* 11b *");
    println!("My solution: {sol_11b}");
    println!("Correct solution: {correct_sol_11b}");
    println!("Equal: {}\n", sol_11b == correct_sol_11b);
}

/// Returns the expansion of the grid in the given axis, as a running sum
fn expansion(grid: &Array2<bool>, axis: usize) -> Vec<usize> {
    grid.axis_iter(Axis(axis))
        .scan(0, |acc, row| {
            if row.iter().all(|&cell| !cell) {
                *acc += 1;
            }
            Some(*acc)
        })
        .collect()
}

fn solve(
    grid: &Array2<bool>,
    expansion_x: &[usize],
    expansion_y: &[usize],
    expansion_factor: usize,
) -> usize {
    grid.indexed_iter()
        .filter_map(|(idx, &cell)| cell.then_some(idx))
        .tuple_combinations()
        .map(|((y1, x1), (y2, x2))| {
            let (min_x, min_y) = (x1.min(x2), y1.min(y2));
            let (max_x, max_y) = (x1.max(x2), y1.max(y2));
            let delta_x = expansion_x[max_x] - expansion_x[min_x];
            let delta_y = expansion_y[max_y] - expansion_y[min_y];
            max_x - min_x + max_y - min_y + (delta_x + delta_y) * (expansion_factor - 1)
        })
        .sum()
}
