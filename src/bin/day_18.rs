use advent_of_code_2023_in_rust::parse_regex;
use ndarray::Array2;
use regex::Regex;
use std::collections::HashSet;
use std::ops::{Index, IndexMut, RangeInclusive};

fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_18_test.txt");
    //let input = include_str!("../../puzzle_inputs/day_18.txt");

    //println!("input len: {}", input.len());
    //println!("input:\n{}", input);

    let pattern = r"(R|L|D|U) (\d+) \(#([0-9,a-f]{6})\)";
    let re = Regex::new(pattern).unwrap();

    let mut dug = HashSet::new();
    let mut corners = Vec::new();
    let mut pos = (0, 0);
    dug.insert(pos);
    corners.push(pos);
    for line in parse_regex::parse_lines(re, input) {
        let (dir, steps, _color): (char, i32, &str) = line;
        let (dx, dy) = match dir {
            'R' => (1, 0),
            'L' => (-1, 0),
            'D' => (0, 1),
            'U' => (0, -1),
            _ => panic!("Invalid direction: {}", dir),
        };
        //println!("dir: {}, steps: {}, color: {}", dir, steps, color);
        for _ in 0..steps {
            pos = (pos.0 + dx, pos.1 + dy);
            dug.insert(pos);
        }
        corners.push(pos);
    }

    // Find the bounds
    let min_x = dug.iter().map(|&(x, _)| x).min().unwrap();
    let min_y = dug.iter().map(|&(_, y)| y).min().unwrap();
    let max_x = dug.iter().map(|&(x, _)| x).max().unwrap();
    let max_y = dug.iter().map(|&(_, y)| y).max().unwrap();
    let x_range = min_x..=max_x;
    let y_range = min_y..=max_y;

    // Print the corners
    println!("Corners: {:?}", corners);
    let mut area: isize = 0;
    for ((x1, y1), (x2, y2)) in corners.iter().zip(corners.iter().cycle().skip(1)) {
        area += x1 * y2 - x2 * y1
    }
    let answer = area.abs() / 2;
    println!("Answer: {}", answer);
    panic!("Stop here");

    //// Print in out
    //println!("Before raseterization");
    //for y in x_range.clone() {
    //    for x in y_range.clone() {
    //        let c = if dug.contains(&(x, y)) { '#' } else { '.' };
    //        print!("{}", c);
    //    }
    //    println!();
    //}
    //println!();

    let mut grid: OffsetArray2<CellType> = OffsetArray2::new(x_range.clone(), y_range.clone());
    dug.iter()
        .for_each(|&(x, y)| grid[(x, y)] = CellType::Trench);

    // Flood fill the exterior
    let min_x_edge = y_range.clone().map(|y| (min_x, y));
    let max_x_edge = y_range.clone().map(|y| (max_x, y));
    let min_y_edge = x_range.clone().map(|x| (x, min_y));
    let max_y_edge = x_range.clone().map(|x| (x, max_y));
    let mut exterior: Vec<(isize, isize)> = min_x_edge
        .chain(max_x_edge)
        .chain(min_y_edge)
        .chain(max_y_edge)
        .filter(|pos| grid[*pos] != CellType::Trench)
        .collect();
    exterior
        .iter()
        .for_each(|&pos| grid[pos] = CellType::Exterior);
    //println!("Exterior: {:?}", exterior);
    //println!("Before flood fill");
    //grid.print();
    while let Some((x, y)) = exterior.pop() {
        let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        for neighbor in neighbors {
            //println!(
            //    "neighbor: {:?} in_x: {} in_y: {} x_range: {:?} y_range: {:?}",
            //    neighbor,
            //    x_range.contains(&n_x),
            //    y_range.contains(&y),
            //    x_range,
            //    y_range,
            //);
            if x_range.contains(&neighbor.0)
                && y_range.contains(&neighbor.1)
                && grid[neighbor] == CellType::Unknown
            {
                grid[neighbor] = CellType::Exterior;
                exterior.push(neighbor);
            }
        }
    }
    //println!("After flood fill");
    //grid.print();

    let answer = grid
        .array
        .iter()
        .filter(|&&cell| cell != CellType::Exterior)
        .count();
    println!("Answer: {}", answer);
    //.flatten()
    //.filter(|pos| grid[pos] != CellType::Trench)
    //.collect();

    //// Raseterize the interior
    //for y in min_y..=max_y {
    //    let mut interior = false;
    //    let mut last_trench = false;
    //
    //    for x in min_x..=max_x {
    //        if dug.contains(&(x, y)) {
    //            if !last_trench {
    //                interior = !interior;
    //                last_trench = true;
    //            }
    //        } else {
    //            if interior {
    //                dug.insert((x, y));
    //            }
    //            last_trench = false;
    //        }
    //    }
    //}

    //// Raseterize the interior
    //let mut interior = HashSet::new();
    //for y in min_y..=max_y {
    //    let mut in_interior = false;
    //
    //    for x in min_x..=max_x {
    //        if dug.contains(&(x - 1, y)) && !dug.contains(&(x, y)) {
    //            in_interior = !in_interior;
    //        }
    //        if in_interior && !dug.contains(&(x, y)) {
    //            interior.insert((x, y));
    //        }
    //    }
    //}
    //dug.extend(interior);
    //
    //// Print in out
    //println!("After raseterization");
    //for y in min_y..=max_y {
    //    for x in min_x..=max_x {
    //        let c = if dug.contains(&(x, y)) { '#' } else { '.' };
    //        print!("{}", c);
    //    }
    //    println!();
    //}
    //println!();

    //// Solve 18a
    //let sol_18a: usize = 12;
    //let correct_sol_18a: usize = 32;
    //println!("* 18a *");
    //println!("My solution: {sol_18a}");
    //println!("Correct solution: {correct_sol_18a}");
    //println!("Equal: {}\n", sol_18a == correct_sol_18a);
    //
    //// Solve 18b
    //let sol_18b: usize = 56;
    //let correct_sol_18b: usize = 79;
    //println!("* 18b *");
    //println!("My solution: {sol_18b}");
    //println!("Correct solution: {correct_sol_18b}");
    //println!("Equal: {}\n", sol_18b == correct_sol_18b);
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
enum CellType {
    #[default]
    Unknown,
    Interior,
    Exterior,
    Trench,
}

//impl Default for CellType {
//    fn default() -> Self {
//        CellType::Unknown
//    }
//}

struct OffsetArray2<T> {
    array: Array2<T>,
    row_offset: isize,
    col_offset: isize,
}

impl<T> OffsetArray2<T>
where
    T: Default + Clone,
{
    // Constructor that accepts RangeInclusive for rows and columns
    fn new(row_range: RangeInclusive<isize>, col_range: RangeInclusive<isize>) -> Self {
        let nrows = (row_range.end() - row_range.start() + 1) as usize;
        let ncols = (col_range.end() - col_range.start() + 1) as usize;
        let array = Array2::from_elem((nrows, ncols), T::default());

        OffsetArray2 {
            array,
            row_offset: *row_range.start(),
            col_offset: *col_range.start(),
        }
    }

    // Returns the effective dimensions as a pair of inclusive ranges, adjusted for the offset
    fn dims(&self) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
        let rows = self.array.nrows() as isize;
        let cols = self.array.ncols() as isize;
        (
            self.row_offset..=self.row_offset + rows - 1,
            self.col_offset..=self.col_offset + cols - 1,
        )
    }
}

impl OffsetArray2<CellType> {
    fn print(&self) {
        let (x_range, y_range) = self.dims();
        for y in y_range {
            for x in x_range.clone() {
                let c = match self[(x, y)] {
                    CellType::Unknown => '.',
                    CellType::Interior => 'I',
                    CellType::Exterior => 'E',
                    CellType::Trench => '#',
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

// Implement `Index` trait for immutable indexing with offset adjustment
impl<T> Index<(isize, isize)> for OffsetArray2<T> {
    type Output = T;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        let (row, col) = index;
        let adjusted_row = (row - self.row_offset) as usize;
        let adjusted_col = (col - self.col_offset) as usize;
        &self.array[[adjusted_row, adjusted_col]]
    }
}

// Implement `IndexMut` trait for mutable indexing with offset adjustment
impl<T> IndexMut<(isize, isize)> for OffsetArray2<T> {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        let (row, col) = index;
        let adjusted_row = (row - self.row_offset) as usize;
        let adjusted_col = (col - self.col_offset) as usize;
        &mut self.array[[adjusted_row, adjusted_col]]
    }
}
