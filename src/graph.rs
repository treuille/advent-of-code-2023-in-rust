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
    /// The weight of the node, which is used to compute the shortest path.
    fn weight(&self, node: Node) -> Weight;

    /// The neighbors of the node, which are expanded to find the shortest path.
    /// Because neighbors are lazily expanded, the implementor of a `Node` trait
    /// can define O(N) neighbors in O(1) space.
    fn neighbors(&self, node: Node) -> impl Iterator<Item = Node>;

    /// Compute the shortest path from the current node to the target node.
    /// The retured path includes the start node and target nodes.
    fn shortest_path(&self, start: Node, target: Node) -> Vec<Node>
    where
        Node: Hash + Eq + Clone,
        Weight: Ord + Add<Output = Weight> + Copy,
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
            if node == target {
                break;
            }
            min_heap.extend(
                self.neighbors(node.clone())
                    .filter(|neighbor| !dists.contains_key(neighbor))
                    .map(|neighbor| MinHeapEntry {
                        node: neighbor.clone(),
                        parent: Some(node.clone()),
                        dist: dist + self.weight(neighbor),
                    }),
            );
        }
        let mut path: Vec<Node> =
            iter::successors(Some(target.clone()), |node| dists[node].1.clone()).collect();
        path.reverse();
        path
    }
}

/// An entry in the min heap used to compute the shortest path.
#[derive(PartialEq, Eq, Clone)]
struct MinHeapEntry<Node, Weight> {
    node: Node,
    parent: Option<Node>,
    dist: Weight,
}

//impl<Node, Weight> MinHeapEntry<Node, Weight>
//{
//    /// Constructor for the first node in the min heap.
//    fn new(node: Node) -> Self {
//        let dist = node.weight();
//        MinHeapEntry {
//            node,
//            parent: None,
//            dist,
//        }
//    }
//}

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

        let shortest_path = Lattice1D.shortest_path(-5, 5);
        assert!(
            shortest_path.into_iter().eq(-5..=5),
            "Path should run from -5 to 5 increasing by 1."
        )
    }

    // Test that the shortext path beteween two nodes in a 2D lattice graph
    // is computed correctly.
    //#[test]
    //fn test_2d_lattice_graph() {
    //    #[derive(Clone, Copy, Hash, Eq, PartialEq)]
    //    struct Node2D(i8, i8);
    //
    //    impl Node for Node2D {
    //        type Weight = usize;
    //
    //        fn weight(&self) -> Self::Weight {
    //            1
    //        }
    //
    //        fn neighbors(&self) -> impl Iterator<Item = Self> {
    //            (self.0 - 1..=self.0 + 1)
    //                .flat_map(move |x| (self.1 - 1..=self.1 + 1).map(move |y| Node2D(x, y)))
    //        }
    //    }
    //
    //    let shortest_path = Lattice1D.(-5, -5).shortest_path(&Node2D(5, 5));
    //    assert!(
    //        shortest_path.into_iter().eq((-5..=5).map(|i| Node2D(i, i))),
    //        "Path should run from (-5, -5) to (5, 5) by (+1, +1)."
    //    )
    //}
}
