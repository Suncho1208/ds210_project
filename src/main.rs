use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "Wiki-Vote.txt"; // Path to the dataset file

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let from_node: u32 = parts[0].parse()?;
            let to_node: u32 = parts[1].parse()?;
            graph.entry(from_node).or_default().push(to_node);
        }
    }

    // Calculate out-degrees
    let mut out_degrees = HashMap::new();
    for (node, neighbors) in &graph {
        out_degrees.insert(*node, neighbors.len());
    }

    // Calculate in-degrees
    let mut in_degrees = HashMap::new();
    for neighbors in graph.values() {
        for &neighbor in neighbors {
            *in_degrees.entry(neighbor).or_insert(0) += 1;
        }
    }

    // Print first 10 nodes and their degrees
    println!("\nOut-degrees for first 10 nodes:");
    for (node, degree) in out_degrees.iter().take(10) {
        println!("Node {}: {}", node, degree);
    }

    println!("\nIn-degrees for first 10 nodes:");
    for (node, degree) in in_degrees.iter().take(10) {
        println!("Node {}: {}", node, degree);
    }

    Ok(())
}
