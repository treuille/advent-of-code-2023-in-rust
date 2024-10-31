use advent_of_code_2023_in_rust::graph::Graph;
use advent_of_code_2023_in_rust::grid::parse_char_grid;
use ndarray::Array2;
use std::rc::Rc;

fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_17.txt");

    // Solve 17a
    let mut puzzle: Puzzle = Puzzle::from_str(input);
    println!("dim: {:?}", puzzle.grid.dim());
    let sol_17a: u32 = puzzle.solve();
    let correct_sol_17a: u32 = 870;
    println!("* 17a *");
    println!("My solution: {sol_17a}");
    println!("Correct solution: {correct_sol_17a}");
    println!("Equal: {}\n", sol_17a == correct_sol_17a);

    // Solve 17b
    puzzle.part_b = true;
    let sol_17b: u32 = puzzle.solve();
    let correct_sol_17b: u32 = 1063;
    println!("* 17b *");
    println!("My solution: {sol_17b}");
    println!("Correct solution: {correct_sol_17b}");
    println!("Equal: {}\n", sol_17b == correct_sol_17b);
}

struct Puzzle {
    grid: Array2<u32>,
    part_b: bool,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Clone, Hash, Eq, PartialEq)]
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

    fn solve(&self) -> u32 {
        let start_state = PuzzleState::new(0, 0, Dir::Down, 0);
        let (w, h) = self.grid.dim();
        let target_state = |state: &Rc<PuzzleState>| state.pos == (w - 1, h - 1);
        let shortest_path = self.shortest_path(start_state.clone(), target_state);
        shortest_path[1..]
            .iter()
            .map(|s| self.weight(s.clone()))
            .sum()
    }
}

impl Graph<Rc<PuzzleState>, u32> for Puzzle {
    fn weight(&self, state: Rc<PuzzleState>) -> u32 {
        self.grid[state.pos]
    }

    fn neighbors(&self, state: Rc<PuzzleState>) -> impl Iterator<Item = Rc<PuzzleState>> {
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
        .filter_map({
            let state = Rc::clone(&state);
            move |mut next_state| {
                if next_state.dir == state.dir {
                    next_state = Rc::new(PuzzleState {
                        //pos: next_state.pos,
                        //dir: next_state.dir,
                        momentum: state.momentum + 1,
                        ..*next_state
                    });
                } else if self.part_b && state.momentum != 0 && state.momentum <= 3 {
                    return None;
                }
                let max_consecutive = if self.part_b { 10 } else { 3 };
                (next_state.momentum <= max_consecutive).then_some(next_state)
            }
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
    fn new(i: usize, j: usize, dir: Dir, momentum: u8) -> Rc<Self> {
        Rc::new(Self {
            pos: (i, j),
            dir,
            momentum,
        })
    }
}
