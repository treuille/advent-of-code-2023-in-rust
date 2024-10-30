use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::iter;
use std::ops::Add;

/// This is the key trait of this module, which lets you compute shorted
/// paths between nodes in a graph. The key design goal is that you don't need
/// to instantiate the entire graph in memory, only expand the nodes as needed
/// using the `neighbors` method.
///
/// This allows is even to compute shortest paths in infinite graphs, such as
/// a 2D lattice graph, where the nodes are the integer points in the plane.
pub trait Graph<Node, Weight> {
    fn weight(&self, node: Node) -> Weight;

    /// The neighbors of the node, which are expanded to find the shortest path.
    /// Because neighbors are lazily expanded, the implementor of a `Node` trait
    /// can define O(N) neighbors in O(1) space.
    fn neighbors(&self, node: Node) -> impl Iterator<Item = Node>;

    /// Compute the shortest path from the current node to the target node.
    /// The retured path includes the start node and target nodes.
    fn shortest_path<TargetFn>(&self, start: Node, target_fn: TargetFn) -> Vec<Node>
    where
        Node: Hash + Eq + Clone,
        Weight: Ord + Add<Output = Weight> + Copy,
        TargetFn: Fn(&Node) -> bool,
    {
        // For each node, store the distance to the node and the previous node.
        let mut dists: HashMap<Node, (Weight, Option<Node>)> = HashMap::new();

        // Initialize the min heap with the starting node.
        let mut min_heap: BinaryHeap<MinHeapEntry<Node, Weight>> =
            BinaryHeap::from([MinHeapEntry {
                node: start.clone(),
                parent: None,
                dist: self.weight(start),
            }]);

        // Dijkstra's algorithm
        while let Some(MinHeapEntry { node, parent, dist }) = min_heap.pop() {
            if dists.contains_key(&node) {
                continue;
            }
            dists.insert(node.clone(), (dist, parent));
            if target_fn(&node) {
                println!(
                    "solved dists.len()={} min_heap.len()={}",
                    dists.len(),
                    min_heap.len()
                );
                let mut path: Vec<Node> =
                    iter::successors(Some(node), |node| dists[node].1.clone()).collect();
                path.reverse();
                return path;
            }
            min_heap.extend(self.neighbors(node.clone()).map(|neighbor| MinHeapEntry {
                node: neighbor.clone(),
                parent: Some(node.clone()),
                dist: dist + self.weight(neighbor),
            }));
        }
        unreachable!("Target node not reachable from start node.")
    }
}

/// An entry in the min heap used to compute the shortest path.
#[derive(PartialEq, Eq, Clone)]
struct MinHeapEntry<Node, Weight> {
    node: Node,
    parent: Option<Node>,
    dist: Weight,
}

/// Rather than auto-deriving Ord, we implement it manually so as only to
/// take into account the weight, not the node itself. This allows us to use
/// Nodes that don't implement Ord.
impl<Node, Weight> Ord for MinHeapEntry<Node, Weight>
where
    Node: Eq,
    Weight: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist).reverse()
    }
}

/// Default implementation of PartialOrd, which uses the Ord implementation.
impl<Node, Weight> PartialOrd for MinHeapEntry<Node, Weight>
where
    Node: Eq,
    Weight: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Unit tests for the `Node` struct
#[cfg(test)]
mod tests {
    use super::Graph;

    /// Test that the shortext path beteween two nodes in a 1D lattice graph
    /// is computed correctly.
    #[test]
    fn test_1d_lattice_graph() {
        struct Lattice1D;

        impl Graph<i8, usize> for Lattice1D {
            fn weight(&self, _node: i8) -> usize {
                1
            }

            fn neighbors(&self, node: i8) -> impl Iterator<Item = i8> {
                (node - 1)..=(node + 1)
            }
        }

        let start = -5;
        let target = |x: &i8| *x == 5;
        let shortest_path = Lattice1D.shortest_path(start, target);
        assert!(
            shortest_path.into_iter().eq(-5..=5),
            "Path should run from -5 to 5 increasing by 1."
        )
    }

    // Test that the shortext path beteween two nodes in a 2D lattice graph
    // is computed correctly.
    #[test]
    fn test_2d_lattice_graph() {
        struct Lattice2D;

        impl Graph<(i8, i8), usize> for Lattice2D {
            fn weight(&self, _node: (i8, i8)) -> usize {
                1
            }

            fn neighbors(&self, (x, y): (i8, i8)) -> impl Iterator<Item = (i8, i8)> {
                ((x - 1)..=(x + 1)).zip((y - 1)..=(y + 1))
            }
        }

        let start = (-5, -5);
        let target = |(x, y): &(i8, i8)| *x == 5 && *y == 5;
        let shortest_path = Lattice2D.shortest_path(start, target);
        assert!(
            shortest_path.into_iter().eq((-5..=5).map(|x| (x, x))),
            "Path should run from (-5,-5) to (5,5) diagonally by (+1, +1)."
        )
    }
}
