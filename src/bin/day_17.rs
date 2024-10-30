#![allow(unused_imports, dead_code)]

use advent_of_code_2023_in_rust::graph;
use advent_of_code_2023_in_rust::graph::Graph;
use advent_of_code_2023_in_rust::grid::parse_char_grid;
use ndarray::Array2;

fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_17_test.txt");
    //let input = include_str!("../../puzzle_inputs/day_17.txt");

    #[allow(unused_mut)]
    let mut puzzle: Puzzle = Puzzle::from_str(input);
    puzzle.part_b = true;

    let start_state = puzzle.start_state();
    let (w, h) = puzzle.grid.dim();
    let target_state = |state: &PuzzleState| state.pos == (w - 1, h - 1);
    let shortest_path = puzzle.shortest_path(start_state, target_state);

    // Print the puzzle
    #[allow(unused_variables)]
    for (i, row) in puzzle.grid.rows().into_iter().enumerate() {
        for (j, c) in row.into_iter().enumerate() {
            if let Some(s) = shortest_path.iter().find(|s| s.pos == (i, j)) {
                //print!("# ");
                print!("{}<", s.momentum);
            } else {
                print!("{} ", c);
            }
        }
        println!();
    }

    println!(
        "len: {}",
        shortest_path[1..]
            .iter()
            .map(|s| puzzle.weight(s.clone()))
            .sum::<u32>()
    );

    //let n = MyNode(-5, -5);
    //println!("n: {:?}", n);
    //println!("n.weight(): {}", n.weight());
    //let neighbors: Vec<MyNode> = n.neighbors().collect();
    //println!("n.neighbors(): {:?}", neighbors);
    //
    //println!(
    //    "n.shortest_path_to(&MyNode(5)): {:?}",
    //    n.shortest_path_to(&MyNode(5, 5))
    //);

    //let n2: Box<dyn GraphNode<Weight = usize>> = Box::new(Node {});
    //println!("n2: {:?}", n);
    //println!("n2.weight(): {}", n2.weight());
    //println!("n2.neighbors(): {:?}", n2.neighbors().collect::<Vec<_>>());

    //// Solve 17a
    //let sol_17a: usize = 12;
    //let correct_sol_17a: usize = 32;
    //println!("* 17a *");
    //println!("My solution: {sol_17a}");
    //println!("Correct solution: {correct_sol_17a}");
    //println!("Equal: {}\n", sol_17a == correct_sol_17a);
    //
    //// Solve 17b
    //let sol_17b: usize = 56;
    //let correct_sol_17b: usize = 79;
    //println!("* 17b *");
    //println!("My solution: {sol_17b}");
    //println!("Correct solution: {correct_sol_17b}");
    //println!("Equal: {}\n", sol_17b == correct_sol_17b);
}

struct Puzzle {
    grid: Array2<u32>,
    part_b: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct PuzzleState {
    pos: (usize, usize),
    dir: Dir,
    momentum: u8,
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        Self {
            grid: parse_char_grid(input, |c| c.to_digit(10).unwrap()),
            part_b: false,
        }
    }

    fn start_state(&self) -> PuzzleState {
        PuzzleState::new(0, 0, Dir::Down, 0)
    }
}

impl Graph<PuzzleState, u32> for Puzzle {
    fn weight(&self, state: PuzzleState) -> u32 {
        self.grid[state.pos]
    }

    fn neighbors(&self, state: PuzzleState) -> impl Iterator<Item = PuzzleState> {
        let (i, j) = state.pos;
        let (w, h) = self.grid.dim();
        [
            i.checked_sub(1)
                .map(|i| PuzzleState::new(i, j, Dir::Left, 1)),
            (i + 1 < w).then(|| PuzzleState::new(i + 1, j, Dir::Right, 1)),
            j.checked_sub(1)
                .map(|j| PuzzleState::new(i, j, Dir::Down, 1)),
            (j + 1 < h).then(|| PuzzleState::new(i, j + 1, Dir::Up, 1)),
        ]
        .into_iter()
        .flatten()
        .filter_map(move |mut next_state| {
            if next_state.dir == state.dir {
                next_state.momentum = state.momentum + 1;
            } else if self.part_b && state.momentum != 0 && state.momentum < 3 {
                return None;
            }
            let max_consecutive = if self.part_b { 10 } else { 3 };
            (next_state.momentum <= max_consecutive).then_some(next_state)
        })
        .filter(move |next_state| match next_state.dir {
            Dir::Left => state.dir != Dir::Right,
            Dir::Right => state.dir != Dir::Left,
            Dir::Down => state.dir != Dir::Up,
            Dir::Up => state.dir != Dir::Down,
        })
    }
}

impl PuzzleState {
    fn new(i: usize, j: usize, dir: Dir, momentum: u8) -> Self {
        Self {
            pos: (i, j),
            dir,
            momentum,
        }
    }
}
