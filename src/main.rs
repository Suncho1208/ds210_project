use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "Wiki-Vote.txt"; // Path to the dataset file

    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    // Adjacency list to store the graph
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();

    println!("Building the graph...");
    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') {
            continue; // Skip comments
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let from_node: u32 = parts[0].parse()?;
            let to_node: u32 = parts[1].parse()?;
            graph.entry(from_node).or_default().push(to_node);
        }
    }

    println!("Graph successfully built with {} nodes.", graph.len());

    // Perform BFS starting from node 30
    let start_node = 30;
    println!("\nBFS Traversal starting from Node {}:", start_node);
    let visited_count = bfs(&graph, start_node);
    println!(
        "BFS visited {} nodes in the connected component starting from Node {}.\n",
        visited_count, start_node
    );

    // Calculate degree centrality
    let degree_centrality = calculate_degree_centrality(&graph);
    println!("Degree Centrality for first 10 nodes:");
    for (node, degree) in degree_centrality.iter().take(10) {
        println!("Node {}: Degree Centrality {}", node, degree);
    }

    // Calculate average degree centrality
    let avg_centrality = calculate_average_centrality(&degree_centrality);
    println!("\nThe average degree centrality across all nodes is {:.2}", avg_centrality);

    Ok(())
}

// BFS function to traverse the graph and return visited node count
fn bfs(graph: &HashMap<u32, Vec<u32>>, start: u32) -> usize {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    let mut count = 0;

    queue.push_back(start);
    visited.insert(start, true);

    while let Some(node) = queue.pop_front() {
        println!("Visited Node: {}", node);
        count += 1;

        if let Some(neighbors) = graph.get(&node) {
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
fn calculate_degree_centrality(graph: &HashMap<u32, Vec<u32>>) -> HashMap<u32, usize> {
    let mut degree_centrality = HashMap::new();

    // Out-degree
    for (node, neighbors) in graph {
        *degree_centrality.entry(*node).or_insert(0) += neighbors.len();
    }

    // In-degree
    for neighbors in graph.values() {
        for &neighbor in neighbors {
            *degree_centrality.entry(neighbor).or_insert(0) += 1;
        }
    }

    degree_centrality
}

// Calculate average degree centrality
fn calculate_average_centrality(degree_centrality: &HashMap<u32, usize>) -> f64 {
    let total_degree: usize = degree_centrality.values().sum();

    let node_count = degree_centrality.len();
    
    total_degree as f64 / node_count as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_construction() {
        let mut graph = HashMap::new();
        graph.entry(1).or_default().push(2);
        graph.entry(1).or_default().push(3);

        assert_eq!(graph[&1], vec![2, 3]);
    }

    #[test]
    fn test_bfs_count() {
        let mut graph = HashMap::new();
        graph.entry(1).or_default().push(2);
        graph.entry(1).or_default().push(3);
        graph.entry(2).or_default().push(4);

        let visited_count = bfs(&graph, 1);

        assert_eq!(visited_count, 4);
    }

    #[test]
    fn test_average_centrality() {
        let mut graph = HashMap::new();
        graph.entry(1).or_default().push(2);
        graph.entry(1).or_default().push(3);
        graph.entry(2).or_default().push(3);

        let centrality = calculate_degree_centrality(&graph);

        let avg_centrality = calculate_average_centrality(&centrality);

        assert!((avg_centrality - 1.33).abs() < 0.01);
    }
}
