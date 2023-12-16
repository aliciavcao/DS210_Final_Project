use csv::ReaderBuilder;
use petgraph::graph::UnGraph;
use std::fs::File;
use std::io;


use crate::graph_utils::get_or_add_node;

// Function that creates a graph from csv
pub fn read_graph_from_csv(file_path: &str) -> Result<UnGraph<String, ()>, io::Error> {
    // Creates a new undirected graph with string nodes and empty edges
    let mut graph = UnGraph::<String, ()>::default();

    // Attempts to open the file specified by the file path
    if let Ok(file) = File::open(file_path) {
        // Creates a CSV reader from the file
        let mut reader = ReaderBuilder::new().from_reader(file);

        // Iterates over each record (line) in the CSV file
        for result in reader.records() {
            // Checks if the current record is successfully read
            if let Ok(record) = result {
                // Retrieves or adds a node (vertex) in the graph for the first column of the record
                let source = get_or_add_node(&mut graph, record[0].to_string());
                // Retrieves or adds a node (vertex) in the graph for the second column of the record
                let target = get_or_add_node(&mut graph, record[1].to_string());

                // Adds an edge between the source and target nodes in the graph
                graph.add_edge(source, target, ());
                // Adds an edge between the target and source nodes in the graph (undirected graph)
                graph.add_edge(target, source, ());
            }
        }
    } else {
        // Returns an error if the file cannot be opened
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Error reading the file",
        ));
    }
    // Returns the constructed graph if the CSV file is successfully read and processed
    Ok(graph)
}
