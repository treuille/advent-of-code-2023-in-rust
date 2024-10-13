use advent_of_code_2023_in_rust::grid::{neighbors, parse_char_grid};
use itertools::izip;
use ndarray::Array2;
use std::fmt::Display;

#[allow(unused_must_use, unreachable_code)]
fn main() {
    //test_remove_start_cell();
    //unimplemented!();

    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_10.txt");
    //let input = include_str!("../../puzzle_inputs/day_10_test.txt");
    //let input = include_str!("../../puzzle_inputs/day_10_test_2.txt");

    //println!("input len: {}", input.len());
    //println!("input:\n{}", input);

    let ident = |c: char| c;
    let mut grid = parse_char_grid(input, ident); // Solve 10a

    println!("puzzle input:");
    print_grid(&grid);
    println!();

    // Solve the puzzle with a breadth-first search
    let start_pos = remove_start_cell(&mut grid);
    let dists = dists_from_start(&grid, start_pos);

    //let sol_10a: usize = dists.iter().filter_map(|&d| d).max().unwrap();
    //let correct_sol_10a: usize = 6757;
    //println!("* 10a *");
    //println!("My solution: {sol_10a}");
    //println!("Correct solution: {correct_sol_10a}");
    //println!("Equal: {}\n", sol_10a == correct_sol_10a);
    //
    //unimplemented!("Just testing 10a for now");

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

/// Find the start cell and replace it with the correct pipe
#[allow(dead_code)]
fn remove_start_cell(grid: &mut Array2<char>) -> (usize, usize) {
    let (y, x) = grid
        .indexed_iter()
        .find(|&(_, &cell)| cell == 'S')
        .map(|(pos, _)| pos)
        .unwrap();
    let (h, w) = grid.dim();
    grid[(y, x)] = match (
        x > 0 && pipe_neighbors((y, x - 1), grid).contains(&(y, x)), // West
        x < w - 1 && pipe_neighbors((y, x + 1), grid).contains(&(y, x)), // East
        y > 0 && pipe_neighbors((y - 1, x), grid).contains(&(y, x)), // North
        y < h - 1 && pipe_neighbors((y + 1, x), grid).contains(&(y, x)), // South
    ) {
        (true, true, false, false) => '-',
        (true, false, true, false) => 'J',
        (true, false, false, true) => '7',
        (false, true, true, false) => 'L',
        (false, true, false, true) => 'F',
        (false, false, true, true) => '|',
        neighbors => unreachable!("impossible neighbors: {:?}", neighbors),
    };
    (y, x)
}

fn test_remove_start_cell() {
    let input = r#"
.........
...F-7...
...|.|...
.F-J.L-7.
.|.....|.
.L-7.F-J.
...|.|...
...L-J...
........."#
        .trim();

    let ident = |c: char| c;
    let grid = parse_char_grid(input, ident); // Solve 10a
    print_grid(&grid);

    let pipe_pos: Vec<(usize, usize)> = grid
        .indexed_iter()
        .filter(|&(_, &cell)| cell != '.')
        .map(|(pos, _)| pos)
        .collect();
    println!("pipe_pos: {:?}", pipe_pos);

    for pos in pipe_pos {
        let mut grid = grid.clone();
        let pipe = grid[pos];
        grid[pos] = 'S';
        let start_pos = remove_start_cell(&mut grid);
        println!("pipe: {}", pipe);
        println!("start_pos: {:?}", start_pos);
        print_grid(&grid);
        println!();
        assert_eq!(pos, start_pos);
        assert_eq!(pipe, grid[pos]);
    }

    unimplemented!("All tests passed!");
}

fn pipe_neighbors(pos: (usize, usize), grid: &Array2<char>) -> Vec<(usize, usize)> {
    let pipe = grid[pos];
    match pipe {
        '.' => Vec::new(),
        'S' => unreachable!("Start cell should have been removed"),
        //'S' => neighbors(pos, grid.dim())
        //    .filter(|&neightbor| pipe_neighbors(neightbor, grid).contains(&pos))
        //    .collect(),
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
fn dists_from_start(grid: &Array2<char>, start_pos: (usize, usize)) -> Array2<Option<usize>> {
    let mut dists = Array2::from_elem(grid.dim(), None);
    let mut to_process: Vec<(usize, usize)> = vec![start_pos];
    for dist in 0.. {
        if to_process.is_empty() {
            break;
        }
        let pos_iter = to_process.into_iter();
        to_process = Vec::new();
        for pos in pos_iter {
            if dists[pos].is_none() {
                dists[pos] = Some(dist);
                to_process.extend(pipe_neighbors(pos, grid));
            }
        }
    }
    dists
}

#[allow(unused_variables, unused_mut)]
fn solve_10b(grid: &Array2<char>, dists: &Array2<Option<usize>>) {
    println!("* 10b *");

    println!("dists:");
    print_dist_grid(dists);
    println!();

    //// Test how rows traverses the grid
    //let mut row_test: Array2<Option<usize>> = Array2::from_elem(grid.dim(), None);
    //println!("row_test strides: {:?}", row_test.strides());
    //println!();
    //
    //for mut row in row_test.rows_mut() {
    //    //println!("row strides: {:?}", row.strides());
    //    for (i, cell) in row.iter_mut().enumerate() {
    //        *cell = Some(i);
    //    }
    //}
    //println!("row_test:");
    //print_dist_grid(&row_test);
    //println!();

    // Now we're going to compute the interior nodes
    let mut interior: Array2<char> = Array2::from_elem(grid.dim(), ' ');

    #[allow(unreachable_code, clippy::match_single_binding, clippy::never_loop)]
    for (mut int_row, grid_row, dist_row) in izip!(interior.rows_mut(), grid.rows(), dists.rows()) {
        let mut interior = false;
        let mut last_pipe = ' ';
        for (int_cell, (grid_cell, dist_cell)) in
            izip!(int_row.iter_mut(), izip!(grid_row.iter(), dist_row.iter()))
        {
            *int_cell = match (last_pipe, dist_cell.map(|_| grid_cell)) {
                //(_, Some('|')) => {
                //    last_pipe = 'x';
                //    interior = !interior;
                //    '*'
                //}
                //(_, Some('-')) => {
                //    last_pipe = 'y';
                //    '*'
                //}
                ('J', Some('J')) => unreachable!("unreachable: JJ"),
                ('7', Some('J')) => unreachable!("unreachable: 7J"),
                ('L', Some('J')) => {
                    last_pipe = ' ';
                    '*'
                }
                ('F', Some('J')) => {
                    last_pipe = ' ';
                    interior = !interior;
                    '*'
                }
                ('J', Some('7')) => unreachable!("unreachable: J7"),
                ('7', Some('7')) => unreachable!("unreachable: 77"),
                ('L', Some('7')) => {
                    last_pipe = ' ';
                    interior = !interior;
                    '*'
                }
                ('F', Some('7')) => {
                    last_pipe = ' ';
                    '*'
                }
                ('J', Some('L')) => unreachable!("unreachable: JL"),
                ('7', Some('L')) => unreachable!("unreachable: 7L"),
                ('L', Some('L')) => unreachable!("unreachable: LL"),
                ('F', Some('L')) => unreachable!("unreachable: FL"),
                ('J', Some('F')) => unreachable!("unreachable: JF"),
                ('7', Some('F')) => unreachable!("unreachable: 7F"),
                ('L', Some('F')) => unreachable!("unreachable: LF"),
                ('F', Some('F')) => unreachable!("unreachable: FF"),
                ('J', Some('-')) => unreachable!("unreachable: J-"),
                ('7', Some('-')) => unreachable!("unreachable: 7-"),
                ('L', Some('-')) => '*',
                ('F', Some('-')) => '*',
                ('J', Some('|')) => {
                    last_pipe = '7';
                    interior = !interior;
                    '*'
                }
                ('7', Some('|')) => {
                    last_pipe = 'J';
                    interior = !interior;
                    '*'
                }
                ('L', Some('|')) => unreachable!("unreachable: L|"),
                ('F', Some('|')) => unreachable!("unreachable: F|"),
                (' ', Some('J')) => unreachable!("unexpected: J"),
                (' ', Some('7')) => unreachable!("unexpected: 7"),
                (' ', Some('L')) => {
                    last_pipe = 'L';
                    '*'
                }
                (' ', Some('F')) => {
                    last_pipe = 'F';
                    '*'
                }
                (' ', Some('|')) => {
                    interior = !interior;
                    '*'
                }
                (' ', Some('-')) => unreachable!("unexpected: -"),
                (' ', None) => match interior {
                    true => 'I',
                    false => 'O',
                },
                x => unimplemented!("Not implemented yet: {:?}", x),
            }
        }
    }

    println!("interior:");
    print_grid(&interior);
    println!();

    // Count the number of interior cells
    let num_interior: usize = interior.iter().filter(|&&cell| cell == 'I').count();
    println!("num_interior: {}", num_interior);
    unimplemented!("Just testing 10b for now");
}
