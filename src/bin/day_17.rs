#![allow(unused_imports)]

use advent_of_code_2023_in_rust::graph;
use advent_of_code_2023_in_rust::graph::Graph;
use advent_of_code_2023_in_rust::grid::parse_char_grid;
use ndarray::Array2;

fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_17_test.txt");
    let grid: Array2<u32> = parse_char_grid(input, |c| c.to_digit(10).unwrap());
    println!("input len: {}", input.len());
    println!("grid:\n{:?}", grid);

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

//struct GridCell(usize, usize);
//
//impl graph::Node for GridCell {
//    type Weight = usize;
//
//    fn weight(&self) -> Self::Weight {
//        1
//    }
//
//    fn neighbors(&self) -> impl Iterator<Item = Self> {
//        (self.0 - 1..=self.0 + 1)
//            .flat_map(move |x| (self.1 - 1..=self.1 + 1).map(move |y| GridCell(x, y)))
//    }
//}
