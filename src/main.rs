use vertex_cover::graphs::algorithms::*;
use vertex_cover::graphs::graph::*;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

const PATH: &str = "in.txt";
const K: usize = 7;

fn main() {
    let graph_1: Graph = load_graph_from_file(PATH);
    let graph_2: Graph = graph_1.clone();

    if let Some(vertex_cover_1) = find_vertex_cover(graph_1, K) {
        println!("Vertex cover with O(K^2) kernelization:");
        for vertex in vertex_cover_1 {
            println!("Vertex: {}", vertex);
        }
    }
    else {
        println!("No solution found!");
    }
    println!();
    if let Some(vertex_cover_2) = find_vertex_cover_with_symplex(graph_2, K, PATH) {
        println!("Vertex cover with O(2K) symplex kernelization:");
        for vertex in vertex_cover_2 {
            println!("Vertex: {}", vertex);
        }
    }
    else {
        println!("No solution found!");
    }
}

pub fn load_graph_from_file(filename: &str) -> Graph {
    let path: &Path = Path::new(filename);
    let file: File = File::open(path).expect("Cannot open file!");
    let mut lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();

    let first_line: String = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let n: usize = parts[0];

    let mut graph: Graph = Graph::new(n);

    for ip in lines.map_while(Result::ok) {
        let edge: Vec<usize> = ip.split_whitespace().map(|s| s.parse().unwrap()).collect();
        if edge.len() == 2 {
            graph.add_edge(edge[0], edge[1]);
        }
    }
    graph
}