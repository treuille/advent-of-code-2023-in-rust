#![allow(
    unused_imports,
    dead_code,
    unused_variables,
    unused_mut,
    unreachable_code
)]

use advent_of_code_2023_in_rust::graph;
use advent_of_code_2023_in_rust::graph::Graph;
use advent_of_code_2023_in_rust::grid::parse_char_grid;
use ndarray::Array2;
use std::collections::HashSet;

fn main() {
    // Parse the input, counting the number of matches per card
    //let input = include_str!("../../puzzle_inputs/day_17_test.txt");
    let input = include_str!("../../puzzle_inputs/day_17.txt");

    #[allow(unused_mut)]
    let mut puzzle: Puzzle = Puzzle::from_str(input);
    puzzle.part_b = true;

    let start_state = puzzle.start_state();
    let (w, h) = puzzle.grid.dim();
    let target_state = |state: &PuzzleState| state.pos == (w - 1, h - 1);
    let shortest_path = puzzle.shortest_path(start_state.clone(), target_state);

    // Print the puzzle
    //#[allow(unused_variables)]
    //println!("Shortest path A:\n");
    //for (i, row) in puzzle.grid.rows().into_iter().enumerate() {
    //    for (j, c) in row.into_iter().enumerate() {
    //        if let Some(s) = shortest_path.iter().find(|s| s.pos == (i, j)) {
    //            //print!("# ");
    //            print!("{}<", s.momentum);
    //        } else {
    //            print!("{} ", c);
    //        }
    //    }
    //    println!();
    //}
    //println!();
    //
    //println!("Shortest path B:\n");
    //for (i, row) in puzzle.grid.rows().into_iter().enumerate() {
    //    for (j, c) in row.into_iter().enumerate() {
    //        if shortest_path.iter().any(|s| s.pos == (i, j)) {
    //            print!("# ");
    //            //print!("{}<", s.momentum);
    //        } else {
    //            print!("{} ", c);
    //        }
    //    }
    //    println!();
    //}
    //println!();
    //
    println!(
        "shortest_dist: {}\n",
        shortest_path[1..]
            .iter()
            .map(|s| puzzle.weight(s.clone()))
            .sum::<u32>()
    );

    /////////////////////
    //// DEBUG - BEGIN //
    /////////////////////
    //
    //let soln = r#"
    //2>>>>>>>>1323
    //32154535v5623
    //32552456v4254
    //34465858v5452
    //45466578v>>>>
    //143859879845v
    //445787698776v
    //363787797965v
    //465496798688v
    //456467998645v
    //122468686556v
    //254654888773v
    //432267465553v"#
    //    .trim();
    //let soln_path: HashSet<PuzzleState> = parse_char_grid(soln, |c| c)
    //    .indexed_iter()
    //    .filter_map(|(pos, &c)| {
    //        if c == '>' {
    //            Some(PuzzleState::new(pos.0, pos.1, Dir::Right, 1))
    //        } else if c == 'v' {
    //            Some(PuzzleState::new(pos.0, pos.1, Dir::Down, 1))
    //        } else {
    //            None
    //        }
    //    })
    //    .collect();
    //println!("soln:\n{}", soln);
    //println!("soln_path:\n{:?}", soln_path);
    //println!(
    //    "soln_dist: {}",
    //    soln_path
    //        .iter()
    //        .map(|s| puzzle.weight(s.clone()))
    //        .sum::<u32>()
    //);
    //println!("soln_len: {}", soln_path.len());
    //
    //let mut state = start_state.clone();
    //let supposed_next_state = PuzzleState {
    //    pos: (0, 1),
    //    dir: Dir::Right,
    //    momentum: 1,
    //};
    //println!("start_state: {:?}", start_state);
    //println!("supposed_next_state: {:?}", supposed_next_state);
    //let mut start_neighbors = puzzle.neighbors(start_state.clone());
    //println!("part_b: {}", puzzle.part_b);
    //let next_state = start_neighbors
    //    .find(|state| state.pos == supposed_next_state.pos && state.dir == supposed_next_state.dir);
    //println!("next_state: {:?}", next_state);
    //assert!(next_state.is_some());
    //
    //let mut i = 0;
    //loop {
    //    println!(
    //        "{} found: {:?} weight={}",
    //        i,
    //        state.clone(),
    //        puzzle.weight(state.clone())
    //    );
    //    i += 1;
    //    if let Some(next_state) = puzzle.neighbors(state).find(|neighbor| {
    //        soln_path
    //            .iter()
    //            .any(|soln_state| soln_state.pos == neighbor.pos && soln_state.dir == neighbor.dir)
    //    }) {
    //        state = next_state;
    //    } else {
    //        println!("no subsequent states found");
    //        break;
    //    }
    //}
    //
    ////panic!("Wait. Did this just work AGAIN?");
    //
    ///////////////////
    //// DEBUG - END //
    ///////////////////

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
        // start_state: PuzzleState { pos: (0, 0), dir: Down, momentum: 0 }
        // supposed_next_state: PuzzleState { pos: (0, 1), dir: Right, momentum: 1 }

        let (i, j) = state.pos;
        let (w, h) = self.grid.dim();
        [
            i.checked_sub(1).map(|i| PuzzleState::new(i, j, Dir::Up, 1)),
            (i + 1 < w).then(|| PuzzleState::new(i + 1, j, Dir::Down, 1)),
            j.checked_sub(1)
                .map(|j| PuzzleState::new(i, j, Dir::Left, 1)),
            (j + 1 < h).then(|| PuzzleState::new(i, j + 1, Dir::Right, 1)),
        ]
        .into_iter()
        .flatten()
        .filter_map(move |mut next_state| {
            if next_state.dir == state.dir {
                next_state.momentum = state.momentum + 1;
            } else if self.part_b && state.momentum != 0 && state.momentum <= 3 {
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
