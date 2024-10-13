use advent_of_code_2023_in_rust::grid::parse_char_grid;
use itertools::Itertools;

#[allow(unused_imports)]
use ndarray::Array2;

#[allow(unused_mut, unreachable_code)]
fn main() {
    //test_dist();
    //unimplemented!("just testing");

    let input = include_str!("../../puzzle_inputs/day_11.txt");
    //let input = include_str!("../../puzzle_inputs/day_11_test.txt");
    let parse_cell = |c: char| match c {
        '.' => false,
        '#' => true,
        _ => panic!("Invalid cell: {}", c),
    };
    let mut grid = parse_char_grid(input, parse_cell);
    let expansion_factor = 1000000;

    println!("input len: {}", input.len());
    println!("input:\n{}", input);

    println!("grid: {:?}", grid);
    println!("grid shape: {:?}", grid.shape());
    print_grid(&grid);

    //let mut empty_rows = 0;
    //let grid2: Vec<bool> = grid
    //    .rows()
    //    .into_iter()
    //    .flat_map(|row| {
    //        if row.iter().all(|&cell| !cell) {
    //            empty_rows += 1;
    //            vec![row, row]
    //        } else {
    //            vec![row]
    //        }
    //    })
    //    .flatten()
    //    .copied()
    //    .collect();
    //let grid2 =
    //    Array2::from_shape_vec((grid.shape()[0] + empty_rows, grid.shape()[1]), grid2).unwrap();
    //println!("grid2:");
    //print_grid(&grid2);

    let expansion_x: Vec<usize> = grid
        .columns()
        .into_iter()
        .scan(0, |acc, col| {
            if col.iter().all(|&cell| !cell) {
                *acc += 1;
            }
            Some(*acc)
        })
        .collect();
    println!("expansion_x: {:?}", expansion_x);

    let expansion_y: Vec<usize> = grid
        .rows()
        .into_iter()
        .scan(0, |acc, row| {
            if row.iter().all(|&cell| !cell) {
                *acc += 1;
            }
            Some(*acc)
        })
        .collect();
    println!("expansion_y: {:?}", expansion_y);

    //let min_dists: usize = grid2
    //    .indexed_iter()
    //    .filter_map(|(idx, &cell)| cell.then_some(idx))
    //    .tuple_combinations::<(_, _)>()
    //    .enumerate()
    //    //.map(|(iter, ((y1, x1), (y2, x2)))| {
    //    .map(|(iter, ((y1, x1), (y2, x2)))| {
    //        //let min_x = x1.min(x2);
    //        //let max_x = x1.max(x2);
    //        //let min_y = y1.min(y2);
    //        //let max_y = y1.max(y2);
    //        //min_y += expansion[min_y];
    //        //max_y += expansion[max_y];
    //        let dist = distance((y1, x1), (y2, x2), None);
    //        println!("{}: ({}, {}) -> ({}, {}) = {}", iter, x1, y1, x2, y2, dist);
    //        dist
    //    })
    //    .sum();
    //println!("min_dists: {}", min_dists);
    //
    let min_dists: usize = grid
        .indexed_iter()
        .filter_map(|(idx, &cell)| cell.then_some(idx))
        .tuple_combinations::<(_, _)>()
        .map(|((y1, x1), (y2, x2))| {
            //unimplemented!("expansion_factor: {}", expansion_factor);
            distance(
                (y1, x1),
                (y2, x2),
                Some(&expansion_x),
                Some(&expansion_y),
                expansion_factor,
            )
            //let min_x = x1.min(x2);
            //let max_x = x1.max(x2);
            //let mut min_y = y1.min(y2);
            //let mut max_y = y1.max(y2);
            //min_y += expansion[min_y];
            //max_y += expansion[max_y];
            //max_x - min_x + max_y - min_y
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

// Your puzzle answer was 10033566.
// Your puzzle answer was 560822911938.

fn print_grid(grid: &Array2<bool>) {
    for row in grid.rows() {
        for cell in row {
            print!("{}", if *cell { '#' } else { '.' });
        }
        println!();
    }
}

fn distance(
    (y1, x1): (usize, usize),
    (y2, x2): (usize, usize),
    expansion_x: Option<&[usize]>,
    expansion_y: Option<&[usize]>,
    expansion_factor: usize,
) -> usize {
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);
    let delta_expansion_x = match expansion_x {
        Some(expansion) => expansion[max_x] - expansion[min_x],
        None => 0,
    };
    let delta_expansion_y = match expansion_y {
        Some(expansion) => expansion[max_y] - expansion[min_y],
        None => 0,
    };
    max_x - min_x + max_y - min_y + (delta_expansion_x + delta_expansion_y) * (expansion_factor - 1)
}

//#[allow(dead_code)]
//fn test_dist() {
//    let input: &str = r#"
//....1........
//.........2...
//3............
//.............
//.............
//........4....
//.5...........
//.##.........6
//..##.........
//...##........
//....##...7...
//8....9......."#
//        .trim();
//    println!("input: {}", input);
//    let digit_cell = |c: char| c.to_digit(10);
//    let grid = parse_char_grid(input, digit_cell);
//    println!("grid: {:?}", grid);
//    let find_cell = |d: u32| {
//        grid.indexed_iter()
//            .find(|(_, &cell)| cell == Some(d))
//            .unwrap()
//            .0
//    };
//    let pos_1 = find_cell(5);
//    let pos_2 = find_cell(9);
//    println!("pos_1: {:?}", pos_1);
//    println!("pos_2: {:?}", pos_2);
//    println!("dist: {:?}", distance(pos_1, pos_2, None, None, 1));
//}
