
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
// mod graph;
// use graph::CustomGraph;

// #[derive(Debug)]
// struct Graph<T> {
//     nodes: Vec<T>,
//     edges: Vec<(usize ,usize)>,
//     weights: Vec<u32>,
// }

// level order BFS , tree you get BFS Spanning Tree , cross edges , can start at any vertice , visit all its adjacent vertice before going into the other use queu
// Preorder DFS , use stack , suspend adjecent and explore visited vertice

#[derive (Clone, Debug)]
struct Animal {
    name: String,
  habitat: String,

}
#[derive(Clone, Debug)]
struct Path {
    distance: u32, // distance can represent the length of the path in meters
}
fn main() {
    let mut zoo = Graph::<Animal,Path, Undirected>::new_undirected();
    let lion = zoo.add_node(Animal { name: "Lion".to_string(), habitat: "Savanna".to_string() });
    let zebra = zoo.add_node(Animal { name: "Zebra".into(), habitat: "Savannah".into() });
    let penguin = zoo.add_node(Animal { name: "Penguin".into(), habitat: "Arctic".into() });
    // Add habitats as nodes
    let savannah = zoo.add_node(Animal { name: "Savannah".into(), habitat: "".into() });
    let arctic = zoo.add_node(Animal { name: "Arctic".into(), habitat: "".into() });
    zoo.add_edge(lion, zebra, Path { distance: 100 });
    zoo.add_edge(zebra, savannah, Path { distance: 20 });
    zoo.add_edge(penguin, arctic, Path { distance: 30 });
    // Print the graph
    println!("{:?}", zoo);
    // Connect animals to their habitats

    // let custom_graph = CustomGraph::new();
}
