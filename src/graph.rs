use std::collections::{HashMap, VecDeque};

// Define the Graph structure
pub struct Graph {
    pub adjacency_list: HashMap<u32, Vec<u32>>,
}

impl Graph {
    // Create a new empty graph
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    // Add an edge to the graph
    pub fn add_edge(&mut self, from: u32, to: u32) {
        self.adjacency_list.entry(from).or_default().push(to);
    }

    // Perform BFS and return the number of visited nodes
    pub fn bfs(&self, start: u32) -> usize {
        if !self.adjacency_list.contains_key(&start) {
            return 0; // If start node does not exist
        }

        let mut visited: HashMap<u32, bool> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut count = 0;

        queue.push_back(start);
        visited.insert(start, true);

        while let Some(node) = queue.pop_front() {
            println!("Visited Node: {}", node);
            count += 1;

            if let Some(neighbors) = self.adjacency_list.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains_key(&neighbor) {
                        visited.insert(neighbor, true);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        count
    }

    // Calculate degree centrality for all nodes
    pub fn calculate_degree_centrality(&self) -> HashMap<u32, usize> {
        let mut degree_centrality: HashMap<u32, usize> = HashMap::new();

        // Out-degree
        for (node, neighbors) in &self.adjacency_list {
            *degree_centrality.entry(*node).or_insert(0) += neighbors.len();
        }

        // In-degree
        for neighbors in self.adjacency_list.values() {
            for &neighbor in neighbors {
                *degree_centrality.entry(neighbor).or_insert(0) += 1;
            }
        }

        degree_centrality
    }

    // Calculate the average degree centrality
    pub fn calculate_average_centrality(&self) -> f64 {
        let degree_centrality = self.calculate_degree_centrality();
        let total_degree: usize = degree_centrality.values().sum();
        let node_count = degree_centrality.len();

        if node_count == 0 {
            return 0.0;
        }

        total_degree as f64 / node_count as f64
    }
}
