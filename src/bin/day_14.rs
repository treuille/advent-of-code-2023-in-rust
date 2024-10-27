use advent_of_code_2023_in_rust::grid::parse_char_grid;
use ndarray::Array2;

fn main() {
    // Parse the input, find the start cell and calculate the distances from it
    let input = include_str!("../../puzzle_inputs/day_14.txt");
    //let input = include_str!("../../puzzle_inputs/day_14_test_01.txt");
    let mut grid = parse_char_grid(input, |c| c);
    println!("grid:\n{:?}", grid);

    for mut col in grid.axis_iter_mut(ndarray::Axis(1)) {
        println!("col BEF: {}", col.iter().collect::<String>());
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
        println!("col AFT: {}", col.iter().collect::<String>());
        println!();
    }

    println!("grid:\n{:?}", grid);

    // Score the grid
    let score: usize = grid
        .indexed_iter()
        .filter_map(|((i, j), &c)| {
            println!("i: {}, j: {}, c: {}", i, j, c);
            if c == 'O' {
                println!("score: {}", grid.shape()[0] - i);
                Some(grid.shape()[0] - i)
            } else {
                None
            }
        })
        .sum();
    println!("score: {}", score);
    //let start_pos = remove_start_cell(&mut grid);
    //let dists = dists_from_start(&grid, start_pos);

    //// Solve 14
    //let sol_14: usize = 12;
    //let correct_sol_14: usize = 32;
    //println!("* 14 *");
    //println!("My solution: {sol_14}");
    //println!("Correct solution: {correct_sol_14}");
    //println!("Equal: {}\n", sol_14 == correct_sol_14);
    //
    //// Solve 14b
    //let sol_14b: usize = 56;
    //let correct_sol_14b: usize = 79;
    //println!("* 14b *");
    //println!("My solution: {sol_14b}");
    //println!("Correct solution: {correct_sol_14b}");
    //println!("Equal: {}\n", sol_14b == correct_sol_14b);
}
