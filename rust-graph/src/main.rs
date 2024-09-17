
// https://medium.com/coinmonks/unraveling-connections-exploring-graph-theory-with-rust-part-1-7fca51f22c36
// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
// XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

// Graph structure
// 1. Adjacency Matrix
// 2. Adjacency List
// 3. Edge List

// Adjacency Matrix
// 1. Space Complexity: O(V^2)
// 2. Query Complexity: O(1)
// 3. Add Edge: O(1)
// 4. Remove Edge: O(1)
// 5. Check Edge: O(1)
// 6. Print Graph: O(V^2)

// Adjacency List
// 1. Space Complexity: O(V + E)
// 2. Query Complexity: O(V)
// 3. Add Edge: O(1)
// 4. Remove Edge: O(E)
// 5. Check Edge: O(V)
// 6. Print Graph: O(V + E)

// Edge List
// 1. Space Complexity: O(E)
// 2. Query Complexity: O(V)
// 3. Add Edge: O(1)
// 4. Remove Edge: O(E)
// 5. Check Edge: O(V)
// 6. Print Graph: O(E)

// Graph structure implementation
use petgraph::Graph;
use petgraph::Undirected;

#[derive(Debug)]
struct Graph<T> {
    nodes: Vec<T>,
    edges: Vec<(usize ,usize)>,
    weights: Vec<u32>,
}

#[derive (Clone, Debug)]
struct Animal {
    name: String,
  habitat: String,

}
struct Path {
    distance: u32, // distance can represent the length of the path in meters
}
fn main() {
    let mut zoo = Graph::<Animal,Path, Undirected>::new_undirected();
}
