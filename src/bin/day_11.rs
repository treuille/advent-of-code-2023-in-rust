use advent_of_code_2023_in_rust::grid::parse_char_grid;
use itertools::Itertools;

#[allow(unused_imports)]
use ndarray::Array2;

// .5...........
// .##.........6
// ..##.........
// ...##........
// ....##...7...
// 8....9.......

//   01234
// 0 *....
// 1 ##...
// 2 .##..
// 3 ..##.
// 4 ...##
// 5 ....*

//   01234
// 0 *....
// 1 #....
// 2 #....
// 3 #....
// 4 #....
// 5 .####

// (0,0) -> (4,5) = 9

// distance = 9

#[allow(unused_mut)]
fn main() {
    let input = include_str!("../../puzzle_inputs/day_11_test.txt");
    let parse_cell = |c: char| match c {
        '.' => false,
        '#' => true,
        _ => panic!("Invalid cell: {}", c),
    };
    let mut grid = parse_char_grid(input, parse_cell);

    println!("input len: {}", input.len());
    println!("input:\n{}", input);

    println!("grid: {:?}", grid);
    println!("grid shape: {:?}", grid.shape());
    print_grid(&grid);

    let mut empty_rows = 0;
    let grid2: Vec<bool> = grid
        .rows()
        .into_iter()
        .map(|row| {
            if row.iter().all(|&cell| !cell) {
                empty_rows += 1;
                vec![row, row]
            } else {
                vec![row]
            }
        })
        .flatten()
        .flatten()
        .copied()
        .collect();
    let grid2 =
        Array2::from_shape_vec((grid.shape()[0] + empty_rows, grid.shape()[1]), grid2).unwrap();
    println!("grid2:");
    print_grid(&grid2);

    let expansion: Vec<usize> = grid
        .rows()
        .into_iter()
        .scan(0, |acc, row| {
            if row.iter().all(|&cell| !cell) {
                *acc += 1;
            }
            Some(*acc)
        })
        .collect();
    println!("expansion: {:?}", expansion);

    let min_dists: usize = grid2
        .indexed_iter()
        .filter_map(|(idx, &cell)| cell.then_some(idx))
        .tuple_combinations::<(_, _)>()
        .map(|((y1, x1), (y2, x2))| {
            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let mut min_y = y1.min(y2);
            let mut max_y = y1.max(y2);
            //min_y += expansion[min_y];
            //max_y += expansion[max_y];
            max_x - min_x + max_y - min_y
        })
        .sum();
    println!("min_dists: {}", min_dists);

    let min_dists: usize = grid
        .indexed_iter()
        .filter_map(|(idx, &cell)| cell.then_some(idx))
        .tuple_combinations::<(_, _)>()
        .map(|((y1, x1), (y2, x2))| {
            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let mut min_y = y1.min(y2);
            let mut max_y = y1.max(y2);
            min_y += expansion[min_y];
            max_y += expansion[max_y];
            max_x - min_x + max_y - min_y
        })
        .sum();
    println!("min_dists: {}", min_dists);
    //
    //// Solve 11a
    //let sol_11a: usize = 12;
    //let correct_sol_11a: usize = 32;
    //println!("* 11a *");
    //println!("My solution: {sol_11a}");
    //println!("Correct solution: {correct_sol_11a}");
    //println!("Equal: {}\n", sol_11a == correct_sol_11a);
    //
    //// Solve 11b
    //let sol_11b: usize = 56;
    //let correct_sol_11b: usize = 79;
    //println!("* 11b *");
    //println!("My solution: {sol_11b}");
    //println!("Correct solution: {correct_sol_11b}");
    //println!("Equal: {}\n", sol_11b == correct_sol_11b);
}

fn print_grid(grid: &Array2<bool>) {
    for row in grid.rows() {
        for cell in row {
            print!("{}", if *cell { '#' } else { '.' });
        }
        println!();
    }
}
