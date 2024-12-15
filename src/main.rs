use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "Wiki-Vote.txt"; // Path to the dataset file

    // Open the file and create a buffered reader
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Adjacency list to store the graph
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') {
            continue; // Skip comment lines
        }

        // Split each line into two parts: FromNodeId and ToNodeId
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let from_node: u32 = parts[0].parse()?;
            let to_node: u32 = parts[1].parse()?;
            graph.entry(from_node).or_default().push(to_node);
        }
    }

    // Print first 10 nodes and their edges
    println!("First 10 nodes and their neighbors:");
    for (node, neighbors) in graph.iter().take(10) {
        println!("Node {}: {:?}", node, neighbors);
    }

    Ok(())
}
