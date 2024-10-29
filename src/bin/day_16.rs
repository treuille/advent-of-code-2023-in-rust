use advent_of_code_2023_in_rust::grid::parse_char_grid;
use ndarray::Array2;
//
//use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
enum Direction {
    Up = 0,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cell {
    cell_type: char,
    visited: [bool; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Ray {
    loc: (usize, usize),
    direction: Direction,
}

impl Ray {
    fn advance(&self, (w, h): (usize, usize)) -> Option<Self> {
        let (x, y) = self.loc;
        let (dx, dy) = match self.direction {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Down => (-1, 0),
            Direction::Up => (1, 0),
        };
        let new_loc = (x as isize + dx, y as isize + dy);
        if new_loc.0 < 0 || new_loc.0 >= w as isize || new_loc.1 < 0 || new_loc.1 >= h as isize {
            None
        } else {
            Some(Ray {
                loc: (new_loc.0 as usize, new_loc.1 as usize),
                direction: self.direction,
            })
        }
    }
}

fn main() {
    //// Parse the input, counting the number of matches per card
    let _input = r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#
        .trim();

    let input = include_str!("../../puzzle_inputs/day_16.txt").trim();

    let grid = parse_char_grid(input, |c| c);

    // Solve 16a
    let answer = simulate_beam(
        &grid,
        Ray {
            loc: (0, 0),
            direction: Direction::Right,
        },
    );
    println!("Answer: {}", answer);
    //print_grid(&grid, false);
    //println!();

    // Solve 16b
    let (w, h) = grid.dim();
    let edges_0 = (0..w).map(|i| {
        [
            Ray {
                loc: (i, 0),
                direction: Direction::Right,
            },
            Ray {
                loc: (i, h - 1),
                direction: Direction::Left,
            },
        ]
    });
    let edges_1 = (0..h).map(|j| {
        [
            Ray {
                loc: (0, j),
                direction: Direction::Up,
            },
            Ray {
                loc: (w - 1, j),
                direction: Direction::Down,
            },
        ]
    });
    let answer: usize = edges_0
        .chain(edges_1)
        .flatten()
        .map(|ray| simulate_beam(&grid, ray))
        .max()
        .unwrap();

    println!("Answer: {}", answer);

    //let input = include_str!("../../puzzle_inputs/day_08_test.txt");

    //// Solve 16a
    //let sol_16a: usize = 7517;
    //let correct_sol_16a: usize = 32;
    //println!("* 16a *");
    //println!("My solution: {sol_16a}");
    //println!("Correct solution: {correct_sol_16a}");
    //println!("Equal: {}\n", sol_16a == correct_sol_16a);
    //
    //// Solve 16b
    //let sol_16b: usize = 56;
    //let correct_sol_16b: usize = 7741;
    //println!("* 16b *");
    //println!("My solution: {sol_16b}");
    //println!("Correct solution: {correct_sol_16b}");
    //println!("Equal: {}\n", sol_16b == correct_sol_16b);
}

fn simulate_beam(grid: &Array2<char>, start: Ray) -> usize {
    let mut grid = grid.map(|&c| Cell {
        cell_type: c,
        visited: [false; 4],
    });

    let mut rays: Vec<Ray> = vec![start];

    while let Some(ray) = rays.pop() {
        //println!("Ray: {:?}\n", ray);
        //println!("contains: {}\n", contains_ray(&grid, &ray));
        if contains_ray(&grid, &ray) {
            continue;
        }
        add_ray(&mut grid, &ray);
        let split = |loc, dir_1, dir_2| {
            vec![
                Some(Ray {
                    loc,
                    direction: dir_2,
                }),
                Some(Ray {
                    loc,
                    direction: dir_1,
                }),
            ]
        };
        let reflect = |loc, direction| vec![(Ray { loc, direction }).advance(grid.dim())];
        rays.extend(
            (match (grid[ray.loc].cell_type, ray.direction) {
                ('.', _) => vec![ray.advance(grid.dim())],
                ('|', Direction::Up) | ('|', Direction::Down) => vec![ray.advance(grid.dim())],
                ('|', Direction::Left) | ('|', Direction::Right) => {
                    split(ray.loc, Direction::Up, Direction::Down)
                }
                ('-', Direction::Left) | ('-', Direction::Right) => vec![ray.advance(grid.dim())],
                ('-', Direction::Up) | ('-', Direction::Down) => {
                    split(ray.loc, Direction::Left, Direction::Right)
                }
                ('\\', Direction::Up) => reflect(ray.loc, Direction::Right),
                ('\\', Direction::Down) => reflect(ray.loc, Direction::Left),
                ('\\', Direction::Left) => reflect(ray.loc, Direction::Down),
                ('\\', Direction::Right) => reflect(ray.loc, Direction::Up),
                ('/', Direction::Up) => reflect(ray.loc, Direction::Left),
                ('/', Direction::Down) => reflect(ray.loc, Direction::Right),
                ('/', Direction::Left) => reflect(ray.loc, Direction::Up),
                ('/', Direction::Right) => reflect(ray.loc, Direction::Down),
                (cell_type, _) => unreachable!("unexpected cell type: {}", cell_type),
            })
            .into_iter()
            .flatten(),
        );

        //print_grid(&grid, false);
        //println!("rays: {:?}", rays);
        //println!();
    }

    //print_grid(&grid, true);
    grid.iter()
        .filter(|cell| cell.visited.iter().any(|&x| x))
        .count()
}

fn print_grid(grid: &Array2<Cell>, hide_contraptions: bool) {
    for row in grid.rows() {
        for cell in row {
            print!(
                "{}",
                match (
                    cell.cell_type,
                    cell.visited.iter().any(|&x| x),
                    hide_contraptions
                ) {
                    ('.', true, _) => '#',
                    (_, true, true) => '#',
                    (c, _, _) => c,
                }
            )
        }
        println!();
    }
}

fn contains_ray(grid: &Array2<Cell>, ray: &Ray) -> bool {
    grid[ray.loc].visited[ray.direction as usize]
}

fn add_ray(grid: &mut Array2<Cell>, ray: &Ray) {
    grid[ray.loc].visited[ray.direction as usize] = true;
}
