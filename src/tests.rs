use crate::graph::Graph;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_construction() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);

        assert_eq!(graph.adjacency_list[&1], vec![2, 3]);
    }

    #[test]
    fn test_bfs_visits_correct_nodes() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 5);

        let visited_count = graph.bfs(1);
        assert_eq!(visited_count, 5);
    }

    #[test]
    fn test_degree_centrality() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 3);

        let centrality = graph.calculate_degree_centrality();
        assert_eq!(centrality[&1], 2);
        assert_eq!(centrality[&2], 2);
        assert_eq!(centrality[&3], 2);
    }
}
