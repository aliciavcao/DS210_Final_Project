use petgraph::graph::{NodeIndex, UnGraph};
use crate::graph_utils::bfs_shortest_path;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn calculate_average_distance_within_sample(graph: &UnGraph<String, ()>, sample_size: usize) -> f64 {

    // Check if the graph is empty or the sample size is 0, return 0.0 in these cases
    if graph.node_count() == 0 || sample_size == 0 {
        return 0.0;
    }

    let mut rng = thread_rng();

    // Collecting all node indices from the graph into a vector
    let mut node_indices: Vec<NodeIndex> = graph.node_indices().collect();

    // Shuffle the node indices to randomize their order
    node_indices.shuffle(&mut rng);

    // Take a sample of node indices based on the specified sample size
    let sample_nodes = &node_indices[0..sample_size];

    // Initializing a variable to store the total distance within the sample
    let mut total_distance_sample = 0.0;

    // Iterating over the sampled nodes
    for &node in sample_nodes.iter() {
        let distances = bfs_shortest_path(graph, node);

        // Iterating over all node indices to calculate distances and update total distance
        for &target in &node_indices {
            if let Some(distance) = distances.get(&target) {
                total_distance_sample += *distance as f64;
            }
        }
    }

    // Calculating the average distance within the sample by dividing total distance by the product of sample size and total nodes
    total_distance_sample / (sample_size * node_indices.len()) as f64
}

// Function to calculate the average distance between all pairs of nodes 
pub fn calculate_average_distance_between_all_pairs(graph: &UnGraph<String, ()>) -> f64 {
    // Collects all the node indices in the graph into a vector
    let node_indices: Vec<NodeIndex> = graph.node_indices().collect();
    
    // Initializes a variable to store the total distance between all pairs of nodes
    let mut total_distance_all_pairs = 0.0;

    // Iterate through each node in the graph
    for &node in &node_indices {
        // Calculate distances from the current node to all other nodes using BFS
        let distances = bfs_shortest_path(graph, node);
        
        // Iterate through each node again to calculate the total distance to all other nodes
        for &target in &node_indices {
            // Check if there is a distance to the target node
            if let Some(distance) = distances.get(&target) {
                // If distance exists, add it to the total distance between all pairs
                total_distance_all_pairs += *distance as f64;
            }
        }
    }

    // Calculate the average distance between all pairs of nodes and return it
    total_distance_all_pairs / (node_indices.len()*2) as f64
}


// Function to calculate the average shortest path length in an undirected graph
pub fn calculate_average_shortest_path_length(graph: &UnGraph<String, ()>) -> f64 {
    // Check if the graph is empty or the sample size is 0, return 0.0 in these cases
    if graph.node_count() == 0 {
        return 0.0;
    }
    // Collecting all node indices in the graph into a vector
    let node_indices: Vec<NodeIndex> = graph.node_indices().collect();
    
    // Initializing variables to keep track of total shortest path length and total pairs
    let mut total_shortest_path_length = 0;
    let mut total_pairs = 0;

    // Iterating through each node index in the graph
    for &node in &node_indices {
        // Finding shortest path distances from the current node to all other nodes using BFS
        let distances = bfs_shortest_path(graph, node);

        // Iterating through all node indices to calculate the total shortest path length
        for &target in &node_indices {
            // Checking if there exists a distance value for the target node
            if let Some(distance) = distances.get(&target) {
                // Adding the distance to the total shortest path length and incrementing total_pairs
                total_shortest_path_length += distance;
                total_pairs += 1;
            }
        }
    }

    // Calculating the average shortest path length by dividing the total by the number of pairs
    total_shortest_path_length as f64 / total_pairs as f64
}