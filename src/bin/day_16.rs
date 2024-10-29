use advent_of_code_2023_in_rust::grid::parse_char_grid;
use ndarray::Array2;

#[derive(Clone, Copy)]
#[repr(usize)]
enum Dir {
    Up = 0,
    Down,
    Left,
    Right,
}

struct Cell {
    cell_type: char,
    visited: [bool; 4],
}

struct Ray {
    loc: (usize, usize),
    dir: Dir,
}

fn main() {
    //// Parse the input as a giant char grid
    let input = include_str!("../../puzzle_inputs/day_16.txt").trim();
    let grid = parse_char_grid(input, |c| c);

    // Solve 16a
    let sol_16a: usize = solve_16a(&grid);
    let correct_sol_16a: usize = 7517;
    println!("* 16a *");
    println!("My solution: {sol_16a}");
    println!("Correct solution: {correct_sol_16a}");
    println!("Equal: {}\n", sol_16a == correct_sol_16a);

    // Solve 16b
    let sol_16b: usize = solve_16b(&grid);
    let correct_sol_16b: usize = 7741;
    println!("* 16b *");
    println!("My solution: {sol_16b}");
    println!("Correct solution: {correct_sol_16b}");
    println!("Equal: {}\n", sol_16b == correct_sol_16b);
}

fn solve_16a(grid: &Array2<char>) -> usize {
    simulate_ray(grid, Ray::new(0, 0, Dir::Right))
}

fn solve_16b(grid: &Array2<char>) -> usize {
    let (w, h) = grid.dim();
    let edges_0 = (0..w).map(|i| [Ray::new(i, 0, Dir::Right), Ray::new(i, h - 1, Dir::Left)]);
    let edges_1 = (0..h).map(|j| [Ray::new(0, j, Dir::Up), Ray::new(w - 1, j, Dir::Down)]);
    edges_0
        .chain(edges_1)
        .flatten()
        .map(|ray| simulate_ray(grid, ray))
        .max()
        .unwrap()
}

impl Ray {
    fn new(i: usize, j: usize, dir: Dir) -> Self {
        Ray { loc: (i, j), dir }
    }

    fn advance(&self, (w, h): (usize, usize)) -> Option<Self> {
        let (i, j) = self.loc;
        match self.dir {
            Dir::Left => j.checked_sub(1).map(|j| Ray::new(i, j, self.dir)),
            Dir::Right => (j + 1 < w).then(|| Ray::new(i, j + 1, self.dir)),
            Dir::Down => i.checked_sub(1).map(|i| Ray::new(i, j, self.dir)),
            Dir::Up => (i + 1 < h).then(|| Ray::new(i + 1, j, self.dir)),
        }
    }
}

fn simulate_ray(grid: &Array2<char>, start: Ray) -> usize {
    let mut grid = grid.map(|&c| Cell {
        cell_type: c,
        visited: [false; 4],
    });

    let mut rays: Vec<Ray> = vec![start];

    while let Some(ray) = rays.pop() {
        if grid[ray.loc].visited[ray.dir as usize] {
            continue;
        }
        grid[ray.loc].visited[ray.dir as usize] = true;
        let split = |(i, j), d1, d2| vec![Some(Ray::new(i, j, d1)), Some(Ray::new(i, j, d2))];
        let reflect = |(i, j), dir| vec![Ray::new(i, j, dir).advance(grid.dim())];
        rays.extend(
            (match (grid[ray.loc].cell_type, ray.dir) {
                ('.', _) => vec![ray.advance(grid.dim())],
                ('|', Dir::Up) | ('|', Dir::Down) => vec![ray.advance(grid.dim())],
                ('|', Dir::Left) | ('|', Dir::Right) => split(ray.loc, Dir::Up, Dir::Down),
                ('-', Dir::Left) | ('-', Dir::Right) => vec![ray.advance(grid.dim())],
                ('-', Dir::Up) | ('-', Dir::Down) => split(ray.loc, Dir::Left, Dir::Right),
                ('\\', Dir::Up) | ('/', Dir::Down) => reflect(ray.loc, Dir::Right),
                ('\\', Dir::Down) | ('/', Dir::Up) => reflect(ray.loc, Dir::Left),
                ('\\', Dir::Left) | ('/', Dir::Right) => reflect(ray.loc, Dir::Down),
                ('\\', Dir::Right) | ('/', Dir::Left) => reflect(ray.loc, Dir::Up),
                (cell_type, _) => unreachable!("unexpected cell: {}", cell_type),
            })
            .into_iter()
            .flatten(),
        );
    }

    grid.iter()
        .filter(|cell| cell.visited.iter().any(|&x| x))
        .count()
}
