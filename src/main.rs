use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "Wiki-Vote.txt"; // Path to the dataset file

    // Open the file to read it
    let file = File::open(file_path)?;
    let reader = BufReader::new(file); // Wrap the file in a buffered reader to read it line by line

    // This is where I’ll store the graph as an adjacency list
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();

    // Go through each line in the file
    for line in reader.lines() {
        let line = line?; // Get the current line (unwrap any errors)

        // If the line starts with '#', skip it (it's just a comment)
        if line.starts_with('#') {
            continue;
        }

        // Split the line into two parts: FromNodeId and ToNodeId
        let parts: Vec<&str> = line.split_whitespace().collect();

        // If there are exactly two parts (a valid edge), process them
        if parts.len() == 2 {
            let from_node: u32 = parts[0].parse()?; // First part: the "from" node
            let to_node: u32 = parts[1].parse()?;   // Second part: the "to" node

            // Add the "to" node to the list of neighbors for the "from" node
            graph.entry(from_node).or_default().push(to_node);
        }
    }

    // Print the first 10 nodes and their neighbors to check if the graph looks right
    for (node, neighbors) in graph.iter().take(10) {
        println!("Node {}: {:?}", node, neighbors);
    }

    Ok(())
}

