use vertex_cover::graphs::algorithms::*;
use vertex_cover::graphs::graph::*;
use std::io::{self, BufRead};
use std::time::{SystemTime};
use std::time::UNIX_EPOCH;
use std::path::Path;
use std::fs::File;

const PATH: &str = "in.txt";
const K: usize = 7;

fn main() {
    let graph_1: Graph = load_graph_from_file(PATH);
    let graph_2: Graph = graph_1.clone();
    let graph_3: Graph = graph_1.clone();
    let graph_4: Graph = graph_1.clone();

    let mut start: std::time::Duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    if let Some(vertex_cover_1) = find_vertex_cover_1(graph_1, K) {
        println!("Vertex cover with O(K^2) kernelization - non optimal fpt:");
        for vertex in vertex_cover_1 {
            println!("Vertex: {}", vertex);
        }
    }
    else {
        println!("No solution found!");
    }
    let mut end: std::time::Duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Solution 1 time: {:?}", end - start);
    println!();

    start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    if let Some(vertex_cover_2) = find_vertex_cover_2(graph_2, K) {
        println!("Vertex cover with O(K^2) kernelization - optimal fpt:");
        for vertex in vertex_cover_2 {
            println!("Vertex: {}", vertex);
        }
    }
    else {
        println!("No solution found!");
    }
    end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Solution 2 time: {:?}", end - start);
    println!();


    start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    if let Some(vertex_cover_3) = find_vertex_cover_with_symplex_1(graph_3, K, PATH) {
        println!("Vertex cover with O(K^2) kernelization - non optimal fpt:");
        for vertex in vertex_cover_3 {
            println!("Vertex: {}", vertex);
        }
    }
    else {
        println!("No solution found!");
    }
    end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Solution 3 time: {:?}", end - start);
    println!();


    start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    if let Some(vertex_cover_4) = find_vertex_cover_with_symplex_2(graph_4, K, PATH) {
        println!("Vertex cover with O(2K) symplex kernelization - optimal fpt:");
        for vertex in vertex_cover_4 {
            println!("Vertex: {}", vertex);
        }
    }
    else {
        println!("No solution found!");
    }
    end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Solution 4 time: {:?}", end - start);
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