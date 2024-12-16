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

    // Build the graph (adjacency list)
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

    // Perform BFS starting from node 30
    let start_node = 30;
    println!("\nBFS Traversal starting from Node {}:", start_node);
    bfs(&graph, start_node);

    Ok(())
}

// BFS function to traverse the graph
fn bfs(graph: &HashMap<u32, Vec<u32>>, start: u32) {
    let mut visited = HashMap::new(); // Track visited nodes
    let mut queue = VecDeque::new(); // Queue for BFS

    queue.push_back(start);
    visited.insert(start, true);

    while let Some(node) = queue.pop_front() {
        println!("Visited Node: {}", node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains_key(&neighbor) {
                    visited.insert(neighbor, true);
                    queue.push_back(neighbor);
                }
            }
        }
    }
}
