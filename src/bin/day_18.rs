#![allow(unused_imports)]
use advent_of_code_2023_in_rust::parse_regex;
use itertools::Itertools;
use ndarray::Array2;
use regex::Regex;
use std::collections::HashSet;
use std::ops::{Index, IndexMut, RangeInclusive};

#[allow(unreachable_code)]
fn main() {
    // Parse the input, counting the number of matches per card
    //let input = include_str!("../../puzzle_inputs/day_18_test.txt");
    let input = include_str!("../../puzzle_inputs/day_18.txt");
    let answer = solve_both(input);
    println!("Answer: {}", answer);
}
//    //println!("input len: {}", input.len());
//    //println!("input:\n{}", input);
//
//    let pattern = r"(R|L|D|U) (\d+) \(#([0-9,a-f]{6})\)";
//    let re = Regex::new(pattern).unwrap();
//
//    let mut dug = HashSet::new();
//    let mut corners = Vec::new();
//    let mut pos = (0, 0);
//    dug.insert(pos);
//    corners.push(pos);
//    for line in parse_regex::parse_lines(re, input) {
//        let (dir, steps, _color): (char, i64, &str) = line;
//        let (dx, dy) = match dir {
//            'R' => (1, 0),
//            'L' => (-1, 0),
//            'D' => (0, 1),
//            'U' => (0, -1),
//            _ => panic!("Invalid direction: {}", dir),
//        };
//        //println!("dir: {}, steps: {}, color: {}", dir, steps, color);
//        for _ in 0..steps {
//            pos = (pos.0 + dx, pos.1 + dy);
//            dug.insert(pos);
//        }
//        corners.push(pos);
//    }
//
//    // Find the bounds
//    let min_x = dug.iter().map(|&(x, _)| x).min().unwrap();
//    let min_y = dug.iter().map(|&(_, y)| y).min().unwrap();
//    let max_x = dug.iter().map(|&(x, _)| x).max().unwrap();
//    let max_y = dug.iter().map(|&(_, y)| y).max().unwrap();
//    let x_range = min_x..=max_x;
//    let y_range = min_y..=max_y;
//
//    // Print the corners
//    println!("Corners: {:?}", corners);
//    let mut area: i64 = 0;
//    for ((x1, y1), (x2, y2)) in corners.iter().zip(corners.iter().cycle().skip(1)) {
//        area += x1 * y2 - x2 * y1
//    }
//    let answer = area.abs() / 2;
//    println!("Answer: {}", answer);
//
//    panic!("Stop here");
//
//    //// Print in out
//    //println!("Before raseterization");
//    //for y in x_range.clone() {
//    //    for x in y_range.clone() {
//    //        let c = if dug.contains(&(x, y)) { '#' } else { '.' };
//    //        print!("{}", c);
//    //    }
//    //    println!();
//    //}
//    //println!();
//
//    let mut grid: OffsetArray2<CellType> = OffsetArray2::new(x_range.clone(), y_range.clone());
//    dug.iter()
//        .for_each(|&(x, y)| grid[(x, y)] = CellType::Trench);
//
//    // Flood fill the exterior
//    let min_x_edge = y_range.clone().map(|y| (min_x, y));
//    let max_x_edge = y_range.clone().map(|y| (max_x, y));
//    let min_y_edge = x_range.clone().map(|x| (x, min_y));
//    let max_y_edge = x_range.clone().map(|x| (x, max_y));
//    let mut exterior: Vec<(i64, i64)> = min_x_edge
//        .chain(max_x_edge)
//        .chain(min_y_edge)
//        .chain(max_y_edge)
//        .filter(|pos| grid[*pos] != CellType::Trench)
//        .collect();
//    exterior
//        .iter()
//        .for_each(|&pos| grid[pos] = CellType::Exterior);
//    //println!("Exterior: {:?}", exterior);
//    //println!("Before flood fill");
//    //grid.print();
//    while let Some((x, y)) = exterior.pop() {
//        let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
//        for neighbor in neighbors {
//            //println!(
//            //    "neighbor: {:?} in_x: {} in_y: {} x_range: {:?} y_range: {:?}",
//            //    neighbor,
//            //    x_range.contains(&n_x),
//            //    y_range.contains(&y),
//            //    x_range,
//            //    y_range,
//            //);
//            if x_range.contains(&neighbor.0)
//                && y_range.contains(&neighbor.1)
//                && grid[neighbor] == CellType::Unknown
//            {
//                grid[neighbor] = CellType::Exterior;
//                exterior.push(neighbor);
//            }
//        }
//    }
//    //println!("After flood fill");
//    //grid.print();
//
//    let answer = grid
//        .array
//        .iter()
//        .filter(|&&cell| cell != CellType::Exterior)
//        .count();
//    println!("Answer: {}", answer);
//    //.flatten()
//    //.filter(|pos| grid[pos] != CellType::Trench)
//    //.collect();
//
//    //// Raseterize the interior
//    //for y in min_y..=max_y {
//    //    let mut interior = false;
//    //    let mut last_trench = false;
//    //
//    //    for x in min_x..=max_x {
//    //        if dug.contains(&(x, y)) {
//    //            if !last_trench {
//    //                interior = !interior;
//    //                last_trench = true;
//    //            }
//    //        } else {
//    //            if interior {
//    //                dug.insert((x, y));
//    //            }
//    //            last_trench = false;
//    //        }
//    //    }
//    //}
//
//    //// Raseterize the interior
//    //let mut interior = HashSet::new();
//    //for y in min_y..=max_y {
//    //    let mut in_interior = false;
//    //
//    //    for x in min_x..=max_x {
//    //        if dug.contains(&(x - 1, y)) && !dug.contains(&(x, y)) {
//    //            in_interior = !in_interior;
//    //        }
//    //        if in_interior && !dug.contains(&(x, y)) {
//    //            interior.insert((x, y));
//    //        }
//    //    }
//    //}
//    //dug.extend(interior);
//    //
//    //// Print in out
//    //println!("After raseterization");
//    //for y in min_y..=max_y {
//    //    for x in min_x..=max_x {
//    //        let c = if dug.contains(&(x, y)) { '#' } else { '.' };
//    //        print!("{}", c);
//    //    }
//    //    println!();
//    //}
//    //println!();
//
//    //// Solve 18a
//    //let sol_18a: usize = 12;
//    //let correct_sol_18a: usize = 32;
//    //println!("* 18a *");
//    //println!("My solution: {sol_18a}");
//    //println!("Correct solution: {correct_sol_18a}");
//    //println!("Equal: {}\n", sol_18a == correct_sol_18a);
//    //
//    //// Solve 18b
//    //let sol_18b: usize = 56;
//    //let correct_sol_18b: usize = 79;
//    //println!("* 18b *");
//    //println!("My solution: {sol_18b}");
//    //println!("Correct solution: {correct_sol_18b}");
//    //println!("Equal: {}\n", sol_18b == correct_sol_18b);
//}
//
//#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
//enum CellType {
//    #[default]
//    Unknown,
//    Interior,
//    Exterior,
//    Trench,
//}
//
////impl Default for CellType {
////    fn default() -> Self {
////        CellType::Unknown
////    }
////}
//
//struct OffsetArray2<T> {
//    array: Array2<T>,
//    row_offset: i64,
//    col_offset: i64,
//}
//
//impl<T> OffsetArray2<T>
//where
//    T: Default + Clone,
//{
//    // Constructor that accepts RangeInclusive for rows and columns
//    fn new(row_range: RangeInclusive<i64>, col_range: RangeInclusive<i64>) -> Self {
//        let nrows = (row_range.end() - row_range.start() + 1) as usize;
//        let ncols = (col_range.end() - col_range.start() + 1) as usize;
//        let array = Array2::from_elem((nrows, ncols), T::default());
//
//        OffsetArray2 {
//            array,
//            row_offset: *row_range.start(),
//            col_offset: *col_range.start(),
//        }
//    }
//
//    // Returns the effective dimensions as a pair of inclusive ranges, adjusted for the offset
//    fn dims(&self) -> (RangeInclusive<i64>, RangeInclusive<i64>) {
//        let rows = self.array.nrows() as i64;
//        let cols = self.array.ncols() as i64;
//        (
//            self.row_offset..=self.row_offset + rows - 1,
//            self.col_offset..=self.col_offset + cols - 1,
//        )
//    }
//}
//
//impl OffsetArray2<CellType> {
//    fn print(&self) {
//        let (x_range, y_range) = self.dims();
//        for y in y_range {
//            for x in x_range.clone() {
//                let c = match self[(x, y)] {
//                    CellType::Unknown => '.',
//                    CellType::Interior => 'I',
//                    CellType::Exterior => 'E',
//                    CellType::Trench => '#',
//                };
//                print!("{}", c);
//            }
//            println!();
//        }
//    }
//}
//
//// Implement `Index` trait for immutable indexing with offset adjustment
//impl<T> Index<(i64, i64)> for OffsetArray2<T> {
//    type Output = T;
//
//    fn index(&self, index: (i64, i64)) -> &Self::Output {
//        let (row, col) = index;
//        let adjusted_row = (row - self.row_offset) as usize;
//        let adjusted_col = (col - self.col_offset) as usize;
//        &self.array[[adjusted_row, adjusted_col]]
//    }
//}
//
//// Implement `IndexMut` trait for mutable indexing with offset adjustment
//impl<T> IndexMut<(i64, i64)> for OffsetArray2<T> {
//    fn index_mut(&mut self, index: (i64, i64)) -> &mut Self::Output {
//        let (row, col) = index;
//        let adjusted_row = (row - self.row_offset) as usize;
//        let adjusted_col = (col - self.col_offset) as usize;
//        &mut self.array[[adjusted_row, adjusted_col]]
//    }
//}
//
fn solve_both(input: &str) -> i64 {
    let pattern = r"(R|L|D|U) (\d+) \(#([0-9,a-f]{5})([0-3])\)";
    let re = Regex::new(pattern).unwrap();
    let line_iter = parse_regex::parse_lines(re, input);

    let mut instructions_a: Vec<_> = Vec::new();
    let mut instructions_b: Vec<_> = Vec::new();

    // Compute the vertices, centered in each grid cell
    for line in line_iter {
        let (dir_a, steps_a, steps_b, dir_b): (char, i64, &str, u8) = line;
        assert!(steps_a > 0, "Steps must be positive: {}", steps_a);
        instructions_a.push((dir_a, steps_a));

        let steps_b = i64::from_str_radix(steps_b, 16).unwrap();
        let dir_b = match dir_b {
            0 => 'R',
            1 => 'D',
            2 => 'L',
            3 => 'U',
            _ => panic!("Invalid direction: {}", dir_b),
        };
        instructions_b.push((dir_b, steps_b));
    }

    let answer_a = solve(instructions_a.into_iter());
    println!("Answer A: {}", answer_a);

    println!("instructions_b: {:?}", instructions_b);
    //panic!("Stop here");
    let answer_b = solve(instructions_b.into_iter());
    println!("Answer B: {}", answer_b);

    // TODO: Fix this return
    answer_a
}

fn solve(instructions: impl Iterator<Item = (char, i64)>) -> i64 {
    let mut centered_vertices = Vec::new();
    let mut pos = (0, 0);
    centered_vertices.push(pos);
    for (dir, steps) in instructions {
        assert!(steps > 0, "Steps must be positive: {}", steps);
        match dir {
            'R' => pos.0 += steps,
            'L' => pos.0 -= steps,
            'D' => pos.1 += steps,
            'U' => pos.1 -= steps,
            _ => panic!("Invalid direction: {}", dir),
        };
        assert!(
            pos != *centered_vertices.last().unwrap(),
            "Cannot move to the same position: pos:{:?}",
            pos
        );
        centered_vertices.push(pos);
    }
    //println!("Centered vertices: {:?}", centered_vertices);

    // Compute the vertices, at the corners of each grid cell
    let mut corner_vertices = Vec::new();
    for (iter, (c1, c2, c3)) in centered_vertices[1..]
        .iter()
        .cycle()
        .tuple_windows()
        .take(centered_vertices.len())
        .enumerate()
    {
        let safe_norm = |x1: i64, x2: i64| {
            let dx = x2 - x1;
            match dx {
                0 => 0,
                _ => dx / dx.abs(),
            }
        };
        // Figure out a local coordinate system for c2 with unit axes
        let dx = (
            safe_norm(c2.0, c3.0),
            //c3.0 - c2.0 / (c3.0 - c2.0).abs(),
            safe_norm(c2.1, c3.1),
            //c3.1 - c2.1 / (c3.1 - c2.1).abs(),
        );
        assert_eq!(
            i64::min(dx.0.abs(), dx.1.abs()),
            0,
            "dx min failure: c1:{:?} c2:{:?} c3:{:?}, dx:{:?}",
            c1,
            c2,
            c3,
            dx
        );
        assert_eq!(
            i64::max(dx.0.abs(), dx.1.abs()),
            1,
            "dx max failure: c1:{:?} c2:{:?} c3:{:?}, dx:{:?}",
            c1,
            c2,
            c3,
            dx
        );

        let dy = (
            safe_norm(c2.0, c1.0),
            //c2.0 - c1.0 / (c2.0 - c1.0).abs(),
            safe_norm(c2.1, c1.1),
            //c2.1 - c1.1 / (c2.1 - c1.1).abs(),
        );
        assert_eq!(i64::min(dy.0.abs(), dy.1.abs()), 0);
        assert_eq!(
            i64::min(dy.0.abs(), dy.1.abs()),
            0,
            "dy min failure: c1:{:?} c2:{:?} c3:{:?}, dy:{:?}",
            c1,
            c2,
            c3,
            dy
        );
        assert_eq!(i64::max(dy.0.abs(), dy.1.abs()), 1);
        assert_eq!(
            i64::max(dy.0.abs(), dy.1.abs()),
            1,
            "dy max failure: c1:{:?} c2:{:?} c3:{:?}, dy:{:?}",
            c1,
            c2,
            c3,
            dy
        );

        let rot_dx = (-dx.1, dx.0);
        assert_eq!(i64::min(rot_dx.0.abs(), rot_dx.1.abs()), 0);
        assert_eq!(i64::max(rot_dx.0.abs(), rot_dx.1.abs()), 1);

        let neg = |v: (i64, i64)| (-v.0, -v.1);
        let d_corner = {
            if rot_dx == dy {
                println!("case A");
                (-dx.0 - dy.0, -dx.1 - dy.1)
            } else if rot_dx == neg(dy) {
                println!("case B");
                (dx.0 + dy.0, dx.1 + dy.1)
            //} else if dx == neg(dy) {
            //    println!("case C");
            //    rot_dx
            } else {
                unimplemented!(
                    "Unexpected corner: c1:{:?} c2:{:?} c3:{:?} dx:{:?} dy:{:?} rot_dx:{:?}",
                    c1,
                    c2,
                    c3,
                    dx,
                    dy,
                    rot_dx
                );
            }
        };
        let corner_vertex = (
            (2 * c2.0 + 1 + d_corner.0) / 2,
            (2 * c2.1 + 1 + d_corner.1) / 2,
        );
        corner_vertices.push(corner_vertex);
        println!(
            "iter:{} c1:{:?} c2:{:?} c3:{:?} dx:{:?} dy:{:?} d_corner:{:?} corner:{:?}",
            iter, c1, c2, c3, dx, dy, d_corner, corner_vertex
        );
    }

    //println!("Corner vertices: {:?}", corner_vertices);

    let mut area: i64 = 0;
    for ((x1, y1), (x2, y2)) in corner_vertices
        .iter()
        .cycle()
        .tuple_windows()
        .take(corner_vertices.len())
    {
        area += x1 * y2 - x2 * y1
    }
    let answer = area.abs() / 2;
    println!("Answer: {}", answer);
    answer
}
