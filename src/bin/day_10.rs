use advent_of_code_2023_in_rust::grid::parse_char_grid;
use ndarray::Array2;

fn main() {
    // Parse the input, find the start cell and calculate the distances from it
    let input = include_str!("../../puzzle_inputs/day_10.txt");
    let mut grid = parse_char_grid(input, |c| c);
    let start_pos = remove_start_cell(&mut grid);
    let dists = dists_from_start(&grid, start_pos);

    // Solve 10a
    let sol_10a: usize = solve_10a(&dists);
    let correct_sol_10a: usize = 6757;
    println!("* 10a *");
    println!("My solution: {sol_10a}");
    println!("Correct solution: {correct_sol_10a}");
    println!("Equal: {}\n", sol_10a == correct_sol_10a);

    // Solve 10b
    let sol_10b: usize = solve_10b(&grid, &dists);
    let correct_sol_10b: usize = 523;
    println!("* 10b *");
    println!("My solution: {sol_10b}");
    println!("Correct solution: {correct_sol_10b}");
    println!("Equal: {}\n", sol_10b == correct_sol_10b);
}

/// Find the start cell and replace it with the correct pipe
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

// Computes distances from the start cell to all other cells along the pipes
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

/// Computers the neighbors of a cell along the pipes
fn pipe_neighbors((y, x): (usize, usize), grid: &Array2<char>) -> Vec<(usize, usize)> {
    let pipe = grid[(y, x)];
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

/// Find the maximum distance from the start cell
fn solve_10a(dists: &Array2<Option<usize>>) -> usize {
    dists.iter().filter_map(|&d| d).max().unwrap()
}

/// Calculate the number of interior cells by rasterizing the grid
fn solve_10b(grid: &Array2<char>, dists: &Array2<Option<usize>>) -> usize {
    let mut num_interior = 0;
    for (grid_row, dist_row) in grid.rows().into_iter().zip(dists.rows()) {
        let (mut interior, mut last_pipe) = (false, ' ');
        for (grid_cell, dist_cell) in grid_row.iter().zip(dist_row.iter()) {
            if dist_cell.is_some() {
                // This means we are at a pipe cell on the path
                (interior, last_pipe) = match (last_pipe, grid_cell) {
                    ('L', 'J') => (interior, ' '),
                    ('F', 'J') => (!interior, ' '),
                    ('L', '7') => (!interior, ' '),
                    ('F', '7') => (interior, ' '),
                    ('L', '-') => (interior, last_pipe),
                    ('F', '-') => (interior, last_pipe),
                    ('J', '|') => (!interior, '7'),
                    ('7', '|') => (!interior, 'J'),
                    (' ', 'L') => (interior, 'L'),
                    (' ', 'F') => (interior, 'F'),
                    (' ', '|') => (!interior, last_pipe),
                    x => unreachable!("unreachable: {:?}", x),
                }
            } else if interior {
                num_interior += 1;
            }
        }
    }
    num_interior
}
