use vertex_cover::kernel::*;
use vertex_cover::graph::*;
use vertex_cover::fpt::*;

use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use std::thread;

const PATH: &str = "420.txt";

fn main() {
    let builder: thread::Builder = thread::Builder::new().stack_size(256 * 1024 * 1024);   
    let handler: thread::JoinHandle<()> = builder.spawn(|| { run_solver(); }).unwrap();
    handler.join().unwrap();
}

fn run_solver() {
    let mut graph: Graph = load_graph_from_file(PATH);
    
    let mut global_cover: Vec<Vertex> = big_kernelization(&mut graph);
    let k_base: Count = global_cover.len();
    
    if graph.is_empty() {
        println!("Graph solved entirely by kernelization!");
        println!("Total cover size: {}", global_cover.len());
        return;
    }

    let remaining_vertices: Count = graph.size();
    let lower_bound: Count = remaining_vertices.div_ceil(2);
    let upper_bound: Count = remaining_vertices;

    println!("Starting FPT phase.");
    println!("Remaining vertices to cover: {}", remaining_vertices);
    println!("Searching in range k = [{} .. {}]", lower_bound, upper_bound);

    for k_remaining in lower_bound..=upper_bound {
        println!("  -> Trying budget k = {} (Total VC = {})", k_remaining, k_base + k_remaining);
        if let Some(fpt_cover) = fpt(graph.clone(), k_remaining) {
            println!("SUCCESS!");
            global_cover.extend(fpt_cover);
            println!("Optimal Vertex Cover found. Total size: {}", global_cover.len());
            return;
        }
    }
    
    println!("Failed to find solution (this shouldn't be reached if graph is valid).");
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
            graph.add_edge(edge[0] - 1, edge[1] - 1);
        }
    }
    graph
}

pub fn big_kernelization(graph: &mut Graph) -> Vec<Vertex> {
    let mut cover: Vec<Vertex> = Vec::new();
    let mut kernelization_not_done: bool = true;

    let mut counter: Count = 0;
    
    while kernelization_not_done {
        kernelization_not_done = false;

        let (partial_cover, v0_count, v12_count, v1_count, n) = kernelization(graph, None); 

        if !partial_cover.is_empty() {
            counter += 1;
            println!("{}. Normal kernelization: |V0| = {}, |V1/2| = {}, |V1| = {}, N = {}, summarise partial cover size = {}",
                counter, v0_count, v12_count, v1_count, n, partial_cover.len() + cover.len());
            cover.extend(partial_cover);
            kernelization_not_done = true;
            continue;
        }
        
        let max_degree_vertex: Vertex = graph.max_degree();
        let (partial_cover, v0_count, v12_count, v1_count, n) = kernelization(graph, Some(max_degree_vertex)); 

        if !partial_cover.is_empty() {
            counter += 1;
            println!("{}. Max degree kernelization: |V0| = {}, |V1/2| = {}, |V1| = {}, N = {}, summarise partial cover size = {}",
                counter, v0_count, v12_count, v1_count, n, partial_cover.len() + cover.len());
            cover.extend(partial_cover);
            kernelization_not_done = true;
            continue;
        }

        let original_vertices: Vec<Vertex> = graph.neighborhood.keys().copied().collect();

        for vertex in original_vertices {
            let (partial_cover, v0_count, v12_count, v1_count, n) = kernelization(graph, None); 
            if !partial_cover.is_empty() {
                counter += 1;
                println!("{}. Naive kernelization for vertex {}: |V0| = {}, |V1/2| = {}, |V1| = {}, N = {}, summarise partial cover size = {}",
                counter, vertex, v0_count, v12_count, v1_count, n, partial_cover.len() + cover.len());
                cover.extend(partial_cover);
                kernelization_not_done = true;
                continue;
            }
        }
    }

    println!("END OF KERNELIZATION");
    println!("Summarize partial cover size: {}", cover.len());
    println!("Number of iterations: {}", counter);

    cover
}