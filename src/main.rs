mod graph; // Import the graph module
mod tests; // Import the tests module


use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use graph::Graph;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "Wiki-Vote.txt"; // Path to the dataset
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut graph = Graph::new();

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
            graph.add_edge(from_node, to_node);
        }
    }
    println!("Graph successfully built with {} nodes.", graph.adjacency_list.len());

    // Perform BFS
    let start_node = 30;
    println!("\nBFS Traversal starting from Node {}:", start_node);
    let visited_count = graph.bfs(start_node);
    println!(
        "BFS visited {} nodes in the connected component starting from Node {}.",
        visited_count, start_node
    );

    // Degree Centrality
    let degree_centrality = graph.calculate_degree_centrality();
    println!("\nDegree Centrality for first 10 nodes:");
    for (node, degree) in degree_centrality.iter().take(10) {
        println!("Node {}: Degree Centrality {}", node, degree);
    }

    // Average Centrality
    let avg_centrality = graph.calculate_average_centrality();
    println!(
        "\nThe average degree centrality across all nodes is {:.5}",
        avg_centrality
    );

    Ok(())
}
