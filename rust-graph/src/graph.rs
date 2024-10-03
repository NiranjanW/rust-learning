pub struct Edge {
    from: usize,
    to: usize,
    weight: f64,
    
}

pub struct CustomGraph <T>{
    nodes :Vec<T>,
    edges: Vec<Edge>
}

  impl<T> CustomGraph<T> {
    pub fn new() -> Self {
        CustomGraph {
            nodes: Vec::new(),
            edges: Vec::new()
        }
    }

    fn add_node(&mut self, node: T) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: f64) {
        self.edges.push(Edge { from, to, weight });
    }

    fn dijkstra(&self, start: usize) -> Vec<f64> {
        let mut distances = vec![f64::INFINITY; self.nodes.len()];
        distances[start] = 0.0;

        let mut visited = vec![false; self.nodes.len()];
 
        for _ in 0..self.nodes.len() {
            let mut min_distance = f64::INFINITY;
            let mut min_index = 0;

            for i in 0..self.nodes.len() {
                if !visited[i] && distances[i] < min_distance {
                    min_distance = distances[i];
                    min_index = i;
                }
            }

            visited[min_index] = true;

            for edge in &self.edges {
                if edge.from == min_index {
                    let new_distance = distances[min_index] + edge.weight;
                    if new_distance < distances[edge.to] {
                        distances[edge.to] = new_distance;
                    }
                }
            }
        }

        distances
    }
    fn neigbour(&self, node: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        for edge in &self.edges {
            if edge.from == node {
                neighbors.push(edge.to);
            }
        }
        neighbors
    } 

    fn neigbours(&self , node:usize) -> Vec<usize>{
        self.edges
        .iter()
        .filter(|edge| edge.from == node)
        .map(|edge| edge.to)
        .collect()
        neighbors
    }
    fn dfs(&self, start: usize, visited: &mut Vec<bool>, result: &mut Vec<usize>) {
        visited[start] = true;
        result.push(start);

        for edge in &self.edges {
            if edge.from == start && !visited[edge.to] {
                self.dfs(edge.to, visited, result);
            }
        }
    }

    fn dfs_recursive(&self,node:usize ,visited:&mut HashSet<usize>,result: &mut Vec<usize>){
        if visited.contains(&node){
            return;
        }
        visited.insert(node);
        result.push(node);

        for &neigbour in self.neighbors
    } 
}
