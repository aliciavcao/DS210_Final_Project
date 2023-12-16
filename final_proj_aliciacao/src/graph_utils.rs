use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::{HashMap, VecDeque};

// Get node index for the given label or add a new node if it doesn't exist
pub fn get_or_add_node(graph: &mut UnGraph<String, ()>, label: String) -> NodeIndex {
    // Check if a node with the same label exists, return its index if found
    if let Some(existing_node) = graph.node_indices().find(|&node| graph[node] == label) {
        existing_node
    } else {
        // If node with the label doesn't exist, add a new node with the label to the graph
        graph.add_node(label)
    }
}

// Breadth-First Search algorithm to find shortest path lengths
pub fn bfs_shortest_path(graph: &UnGraph<String, ()>, start_node: NodeIndex) -> HashMap<NodeIndex, usize> {
    // Initialize a HashMap to store distances
    let mut distances: HashMap<NodeIndex, usize> = HashMap::new(); 
    // Initialize a queue using VecDeque
    let mut queue = VecDeque::new();
    // Enqueue the starting node 
    queue.push_back(start_node); 
// Set the distance of the starting node to 0
    distances.insert(start_node, 0); 

// Process nodes in the queue
    while let Some(node) = queue.pop_front() { 
        // Get the current distance from the start node
        let current_distance = *distances.get(&node).unwrap_or(&0); 
        // Iterate through neighbors of the current node
        for neighbor in graph.neighbors(node) { 
            // Check if neighbor's distance is not calculated yet
            if !distances.contains_key(&neighbor) { 
                // Calculate distance from start node to the neighbor
                distances.insert(neighbor, current_distance + 1); 
                // Enqueue the neighbor for further processing
                queue.push_back(neighbor); 
            }
        }
    }
    // Return HashMap containing shortest path lengths
    distances 
}

