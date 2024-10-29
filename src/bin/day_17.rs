use advent_of_code_2023_in_rust::graph;
use advent_of_code_2023_in_rust::graph::Node;

fn main() {
    //// Parse the input, counting the number of matches per card
    //let input = include_str!("../../puzzle_inputs/day_08_test.txt");
    //println!("input len: {}", input.len());
    //println!("input:\n{}", input);

    let n = MyNode(-5, -5);
    println!("n: {:?}", n);
    println!("n.weight(): {}", n.weight());
    let neighbors: Vec<MyNode> = n.neighbors().collect();
    println!("n.neighbors(): {:?}", neighbors);

    println!(
        "n.shortest_path_to(&MyNode(5)): {:?}",
        n.shortest_path_to(&MyNode(5, 5))
    );

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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct MyNode(i64, i64);

impl graph::Node for MyNode {
    type Weight = usize;

    fn weight(&self) -> Self::Weight {
        1
    }

    fn neighbors(&self) -> impl Iterator<Item = Self> {
        (self.0 - 1..=self.0 + 1)
            .flat_map(move |x| (self.1 - 1..=self.1 + 1).map(move |y| MyNode(x, y)))
    }
}
