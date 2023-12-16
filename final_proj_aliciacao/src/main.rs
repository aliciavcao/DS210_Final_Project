mod graph_utils;
mod file_utils;
mod distance_calculations;

use file_utils::*;
use distance_calculations::*;
use crate::calculate_average_distance_between_all_pairs;

fn main() {
    let file_path = "C:/Users/Alicia/DS210/final_proj_aliciacao/euroroad.csv";

    if let Ok(graph) = read_graph_from_csv(file_path) {
        println!("Number of nodes: {}", graph.node_count());
        println!("Number of edges: {}", graph.edge_count());

        let avg_distance_sample = calculate_average_distance_within_sample(&graph, 500);
        println!("Average distance within the sample: {:.2}
        Sample size: 100", avg_distance_sample);

        let avg_distance_all_pairs = calculate_average_distance_between_all_pairs(&graph);
        println!("Average distance between all node pairs: {:.2}", avg_distance_all_pairs);

        let avg_shortest_path_length = calculate_average_shortest_path_length(&graph);
        println!("Average Shortest Path Length: {:.2}", avg_shortest_path_length);
    } else {
        println!("Error reading the file");
    }
}


// tests

#[cfg(test)]

mod tests {
    use petgraph::graph::UnGraph;
    use crate::calculate_average_distance_between_all_pairs;   
    use crate::calculate_average_distance_within_sample;
    use crate::calculate_average_shortest_path_length;

    #[test]
    fn test_calculate_average_distance() {
        // Create a small graph with known distances
        let mut graph = UnGraph::<String, ()>::new_undirected();
        let node_a = graph.add_node("A".to_string());
        let node_b = graph.add_node("B".to_string());
        let node_c = graph.add_node("C".to_string());

        graph.add_edge(node_a, node_b, ());
        graph.add_edge(node_a, node_c, ());
        graph.add_edge(node_b, node_c, ());

        // Calculate the expected average distance manually
        // For this graph, the distances are: A-B = 1, A-C = 1, B-C = 1
        // Average distance = (1 + 1 + 1) / 3 = 1.0
        let expected_avg_distance = 1.0;

        // Calculate the average distance using the function
        let avg_distance = calculate_average_distance_between_all_pairs(&graph);

        // Assert the expected average distance
        assert_eq!(avg_distance, expected_avg_distance);
    }
    
    // Test case to check if the function returns 0.0 when the sample size is 0
    #[test]
    fn test_zero_sample_size() {
        let graph = UnGraph::<String, ()>::new_undirected();
        assert_eq!(calculate_average_distance_within_sample(&graph, 0), 0.0);
    }

    // Test case to check if the function returns 0.0 for an empty graph
    #[test]
    fn test_empty_graph() {
        let graph = UnGraph::<String, ()>::new_undirected();
        assert_eq!(calculate_average_distance_within_sample(&graph, 5), 0.0);
    }

    //Test case to check if the function returns 0.0 for an empty graph
    #[test]
    fn test_calculate_average_shortest_path_length_empty_graph() {
        let graph: UnGraph<String, ()> = UnGraph::new_undirected();
        assert_eq!(calculate_average_shortest_path_length(&graph), 0.0);
    }

    // Expecting 0 for a graph with a single node
    #[test]
    fn test_calculate_average_shortest_path_length_single_node_graph() {
        let mut graph: UnGraph<String, ()> = UnGraph::new_undirected();
        let _node_a = graph.add_node("A".to_string());
        let result = calculate_average_shortest_path_length(&graph);
        assert_eq!(result, 0.0); 
    }

}

