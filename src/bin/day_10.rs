use advent_of_code_2023_in_rust::grid::{neighbors, parse_char_grid};
use ndarray::Array2;
use std::fmt::Display;

#[allow(unused_must_use)]
fn main() {
    // Parse the input, counting the number of matches per card
    //let input = include_str!("../../puzzle_inputs/day_10.txt");
    //let input = include_str!("../../puzzle_inputs/day_10_test.txt");
    let input = include_str!("../../puzzle_inputs/day_10_test_2.txt");

    //println!("input len: {}", input.len());
    //println!("input:\n{}", input);

    let ident = |c: char| c;
    let grid = parse_char_grid(input, ident); // Solve 10a

    println!("puzzle input:");
    print_grid(&grid);
    println!();

    // Solve the puzzle with a breadth-first search
    let dists = dists_from_start(&grid);

    let sol_10a: usize = dists.iter().filter_map(|&d| d).max().unwrap();
    let correct_sol_10a: usize = 6757;
    println!("* 10a *");
    println!("My solution: {sol_10a}");
    println!("Correct solution: {correct_sol_10a}");
    println!("Equal: {}\n", sol_10a == correct_sol_10a);

    solve_10b(&grid, &dists);

    //// Solve 10b
    //let sol_10b: usize = 56;
    //let correct_sol_10b: usize = 79;
    //println!("* 10b *");
    //println!("My solution: {sol_10b}");
    //println!("Correct solution: {correct_sol_10b}");
    //println!("Equal: {}\n", sol_10b == correct_sol_10b);
}

/// Prints out a grid nice and simply
fn print_grid<T: Display>(grid: &Array2<T>) {
    grid.rows().into_iter().for_each(|row| {
        row.iter().for_each(|cell| print!("{}", cell));
        println!();
    });
}

/// Prints out a grid of distances nice and simply
fn print_dist_grid(dists: &Array2<Option<usize>>) {
    let dist_char: Array2<char> =
        dists.map(|&d| d.map(|d| (d as u8 + b'0') as char).unwrap_or('.'));
    print_grid(&dist_char);
}
/// Find the neighbors of a at a point, givend the pipe structure of the grid
fn pipe_neighbors(pos: (usize, usize), grid: &Array2<char>) -> Vec<(usize, usize)> {
    let pipe = grid[pos];
    match pipe {
        '.' => Vec::new(),
        'S' => neighbors(pos, grid.dim())
            .filter(|&neightbor| pipe_neighbors(neightbor, grid).contains(&pos))
            .collect(),
        _ => {
            let (y, x) = pos;
            let (h, w) = grid.dim();
            [
                (['-', 'J', '7'].contains(&pipe) && x > 0).then(|| (y, x - 1)), // West
                (['-', 'L', 'F'].contains(&pipe) && x < w - 1).then(|| (y, x + 1)), // East
                (['|', 'L', 'J'].contains(&pipe) && y > 0).then(|| (y - 1, x)), // North
                (['|', '7', 'F'].contains(&pipe) && y < h - 1).then(|| (y + 1, x)), // South
            ]
            .into_iter()
            .flatten()
            .collect()
        }
    }
}

/// Calculate the distances from the start point to all other reachable points
fn dists_from_start(grid: &Array2<char>) -> Array2<Option<usize>> {
    let mut dists = Array2::from_elem(grid.dim(), None);
    let mut to_process: Vec<(usize, usize)> = grid
        .indexed_iter()
        .filter(|&(_, &pipe)| pipe == 'S')
        .map(|(pos, _)| pos)
        .collect();
    for dist in 0.. {
        if to_process.is_empty() {
            break;
        }
        let pos_iter = to_process.into_iter();
        to_process = Vec::new();
        for pos in pos_iter {
            if dists[pos].is_none() {
                dists[pos] = Some(dist);
                to_process.extend(pipe_neighbors(pos, &grid));
            }
        }
    }
    dists
}

#[allow(unused_variables)]
fn solve_10b(grid: &Array2<char>, dists: &Array2<Option<usize>>) {
    println!("* 10b *");

    println!("dists:");
    print_dist_grid(dists);
}
