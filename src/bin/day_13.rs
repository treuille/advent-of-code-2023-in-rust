use advent_of_code_2023_in_rust::grid::parse_char_grid;
use ndarray::{Array2, Axis};

fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_13.txt");
    //let input = include_str!("../../puzzle_inputs/day_13_test_0.txt");
    println!("input len: {}", input.len());
    println!("input:\n{}", input);

    println!("* Parsing input *");
    let input = parse_input(input);

    // Start solving 13a

    //// Solve 13a
    let sol_13a = solve_13a(&input);
    let correct_sol_13a: usize = 27300;
    println!("* 13a *");
    println!("My solution: {sol_13a}");
    println!("Correct solution: {correct_sol_13a}");
    println!("Equal: {}\n", sol_13a == correct_sol_13a);

    // Solve 13b
    let sol_13b: usize = solve_13b(&input);
    let correct_sol_13b: usize = 79;
    println!("* 13b *");
    println!("My solution: {sol_13b}");
    println!("Correct solution: {correct_sol_13b}");
    println!("Equal: {}\n", sol_13b == correct_sol_13b);
}

fn solve_13a(input: &[Array2<bool>]) -> usize {
    input
        .iter()
        .map(|grid| {
            // first rows
            let mut axis_counts: [usize; 2] = [0, 0];
            // Process rows
            for axis in 0..2 {
                let shape = grid.shape();
                let outer_dim = shape[axis];

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

                        let row_or_col_2 = grid.index_axis(Axis(axis), idx_2);
                        let row_or_col_3 = grid.index_axis(Axis(axis), idx_3);

                        if row_or_col_2 != row_or_col_3 {
                            break;
                        } else if idx_2 == 0 || (idx_3 + 1) == outer_dim {
                            let delta_count = idx_1 + 1;
                            axis_counts[axis] += delta_count;
                        }
                    }
                }
            }

            axis_counts[0] * 100 + axis_counts[1]
        })
        .sum()
}

#[allow(clippy::wildcard_in_or_patterns)]
fn solve_13b(input: &[Array2<bool>]) -> usize {
    input
        .iter()
        .map(|grid| {
            // first rows
            println!("\ngrid shape: {:?}", grid.shape());
            print_grid(grid);
            let mut axis_counts: [usize; 2] = [0, 0];
            let mut found_along_axes: usize = 0;
            let mut found_smudge: [bool; 2] = [false, false];
            let mut found_reflection: bool = false;
            println!();

            // Process rows
            for axis in 0..2 {
                if found_reflection {
                    println!("Skipping axis {}", axis);
                    continue;
                }
                let shape = grid.shape();
                let outer_dim = shape[axis];

                for idx_1 in 0..(outer_dim - 1) {
                    if found_reflection {
                        println!("Skipping axis={} idx_1={}", axis, idx_1);
                        continue;
                    }
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

                        let row_or_col_2 = grid.index_axis(Axis(axis), idx_2);
                        let row_or_col_3 = grid.index_axis(Axis(axis), idx_3);

                        match row_or_col_2
                            .iter()
                            .zip(row_or_col_3)
                            .filter(|(a, b)| a != b)
                            .count()
                        {
                            0 => {
                                print!(
                                    "AXIS {}: idx_1: {}, offset: {} [{}] == [{}] (no smudge) ",
                                    axis, idx_1, offset, idx_2, idx_3
                                );
                                println!(
                                    "found_smudge: [{}, {}]",
                                    found_smudge[0], found_smudge[1]
                                );
                                if (idx_2 == 0 || (idx_3 + 1) == outer_dim) && found_smudge[axis] {
                                    let delta_count = idx_1 + 1;
                                    axis_counts[axis] += delta_count;
                                    found_along_axes += 1;
                                    println!("AXIS {}: +{}", axis, delta_count);
                                    found_reflection = true;
                                    break;
                                }
                            }
                            1 => {
                                if found_smudge[axis] {
                                    found_smudge[axis] = false;
                                    break;
                                }
                                found_smudge[axis] = true;
                                print!(
                                    "AXIS {}: idx_1: {}, offset: {} [{}] == [{}] (smudge) ",
                                    axis, idx_1, offset, idx_2, idx_3
                                );
                                println!(
                                    "found_smudge: [{}, {}]",
                                    found_smudge[0], found_smudge[1]
                                );
                                if idx_2 == 0 || (idx_3 + 1) == outer_dim {
                                    let delta_count = idx_1 + 1;
                                    axis_counts[axis] += delta_count;
                                    found_along_axes += 1;
                                    println!("AXIS {}: +{}", axis, delta_count);
                                    found_reflection = true;
                                    break;
                                }
                            }
                            _ => {
                                found_smudge[axis] = false;
                                print!(
                                    "AXIS {}: idx_1: {}, offset: {} [{}] != [{}] ",
                                    axis, idx_1, offset, idx_2, idx_3
                                );
                                println!(
                                    "found_smudge: [{}, {}]",
                                    found_smudge[0], found_smudge[1]
                                );
                                break;
                            }
                        }
                    }
                }
            }
            println!("row count: {}", axis_counts[0]);
            println!("col count: {}", axis_counts[1]);
            println!("found_smudge: [{}, {}]", found_smudge[0], found_smudge[1]);
            println!("found_along_axes: {}", found_along_axes);
            println!("found_reflection: {}", found_reflection);
            //assert_eq!(found_along_axes, 1);
            assert!(found_smudge[0] ^ found_smudge[1]);
            assert!(found_reflection);
            let returning = match found_smudge {
                [true, false] => axis_counts[0] * 100,
                [false, true] => axis_counts[1],
                _ => unreachable!(),
            };
            println!("returning: {}", returning);
            returning
        })
        .sum()
}
fn parse_input(input: &str) -> Vec<Array2<bool>> {
    input
        .split("\n\n")
        .map(|s| parse_char_grid(s, |c| c == '#'))
        .collect()
}
fn print_grid(grid: &Array2<bool>) {
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
}
