use advent_of_code_2023_in_rust::grid::parse_char_grid;
use ndarray::{s, Array2};

fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_13.txt");
    //let input = include_str!("../../puzzle_inputs/day_13_test_0.txt");
    println!("input len: {}", input.len());
    println!("input:\n{}", input);

    println!("* Parsing input *");
    let input = parse_input(input);

    // Start solving 13a
    let sol_13a: usize = input
        .iter()
        .map(|grid| {
            // first rows
            println!("\ngrid shape: {:?}", grid.shape());
            print!("  ");
            for i in 0..grid.shape()[1] {
                print!("{}", i % 10);
            }
            println!();
            for (i, row) in grid.outer_iter().enumerate() {
                print!("{} ", i % 10);
                for cell in row {
                    print!("{}", if *cell { '#' } else { '.' });
                }
                println!();
            }
            let mut axis_counts: [usize; 2] = [0, 0];
            let mut found_along_axes: usize = 0;
            println!();

            // Process rows
            for axis in 0..2 {
                let shape = grid.shape();
                let (outer_dim, _) = if axis == 0 {
                    (shape[0], shape[1])
                } else {
                    (shape[1], shape[0])
                };

                for idx_1 in 0..(outer_dim - 1) {
                    for offset in 0.. {
                        let idx_2 = if idx_1 < offset {
                            break;
                        } else {
                            idx_1 - offset
                        };
                        let idx_3 = if idx_1 + offset + 1 >= outer_dim {
                            break;
                        } else {
                            idx_1 + offset + 1
                        };

                        let row_or_col_2 = if axis == 0 {
                            grid.row(idx_2)
                        } else {
                            grid.column(idx_2)
                        };

                        let row_or_col_3 = if axis == 0 {
                            grid.row(idx_3)
                        } else {
                            grid.column(idx_3)
                        };

                        if row_or_col_2 != row_or_col_3 {
                            println!(
                                "AXIS {}: idx_1: {}, offset: {} [{}] != [{}]",
                                axis, idx_1, offset, idx_2, idx_3
                            );
                            break;
                        } else {
                            println!(
                                "AXIS {}: idx_1: {}, offset: {} [{}] == [{}]",
                                axis, idx_1, offset, idx_2, idx_3
                            );
                            if idx_2 == 0 || (idx_3 + 1) == outer_dim {
                                let delta_count = idx_1 + 1;
                                axis_counts[axis] += delta_count;
                                found_along_axes += 1;
                                println!("AXIS {}: +{}", axis, delta_count);
                            }
                        }
                    }
                }
            }

            println!("row count: {}", axis_counts[0]);
            println!("col count: {}", axis_counts[1]);
            println!("found_along_axes: {}", found_along_axes);
            assert_eq!(found_along_axes, 1);
            let returning = axis_counts[0] * 100 + axis_counts[1];
            println!("returning: {}", returning);
            returning
        })
        .sum();

    //// Solve 13a
    let correct_sol_13a: usize = 27300;
    println!("* 13a *");
    println!("My solution: {sol_13a}");
    println!("Correct solution: {correct_sol_13a}");
    println!("Equal: {}\n", sol_13a == correct_sol_13a);

    //// Solve 13b
    //let sol_13b: usize = 56;
    //let correct_sol_13b: usize = 79;
    //println!("* 13b *");
    //println!("My solution: {sol_13b}");
    //println!("Correct solution: {correct_sol_13b}");
    //println!("Equal: {}\n", sol_13b == correct_sol_13b);
}

fn parse_input(input: &str) -> Vec<Array2<bool>> {
    input
        .split("\n\n")
        .map(|s| parse_char_grid(s, |c| c == '#'))
        .collect()
}
