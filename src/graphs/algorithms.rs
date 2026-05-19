use crate::graphs::kernel::*;
use crate::graphs::graph::*;
use std::process::Command;

// ALGORYTM FPT
pub fn fpt_algorithm(mut g: Graph, k: usize) -> Option<Vec<Vertex>> {
    if g.is_empty() {
        return Some(Vec::new());
    }
    if k == 0 {
        return None;
    }
    if let Some((u, v)) = g.get_arbitrary_edge() {   
        let mut g_a: Graph = g.clone();

        g_a.remove_vertex(u);
        
        if let Some(mut cover_a) = fpt_algorithm(g_a, k - 1) {
            cover_a.push(u);
            return Some(cover_a);
        }

        g.remove_vertex(v);
        
        if let Some(mut cover_b) = fpt_algorithm(g, k - 1) {
            cover_b.push(v);
            return Some(cover_b);
        }
    }
    None
}

// ALGORYTM FPT OPTIMAL
pub fn fpt_algorithm_opt(mut g: Graph, k: usize) -> Option<Vec<Vertex>> {
    if g.is_empty() {
        return Some(Vec::new());
    }
    if k == 0 {
        return None;
    }
    if let Some(u) = g.max_degree_vertex() {   

        let neigbors: Vec<usize> = g.get_neighbors(u);
        let degree: usize = neigbors.len();
        if degree == 0 {
            return Some(Vec::new());
        }

        let mut g_a: Graph = g.clone();
        g_a.remove_vertex(u);
        
        if let Some(mut cover_a) = fpt_algorithm_opt(g_a, k - 1) {
            cover_a.push(u);
            return Some(cover_a);
        }

        if degree <= k {
            g.remove_vertex(u);
            for neighbor in &neigbors {
                g.remove_vertex(*neighbor);
            }
            if let Some(mut cover_b) = fpt_algorithm_opt(g, k - 1) {
                cover_b.extend(neigbors);
                return Some(cover_b);
            }
        }
    }
    None
}

// ALGORYTM KERNELIZACJI + FPT ROZMIAR JĄDRA O(K^2)
pub fn find_vertex_cover_1(graph: Graph, k: usize) -> Option<Vec<Vertex>> {
    match kernelization(graph, k) {
        KernelResult::Failed => None,
        KernelResult::Success(instance) => {
            if let Some(mut fpt_cover) = fpt_algorithm(instance.reduced_graph, instance.new_k) {
                fpt_cover.extend(instance.partial_cover);
                Some(fpt_cover)
            } 
            else {
                None
            }
        }
    }
}

pub fn find_vertex_cover_2(graph: Graph, k: usize) -> Option<Vec<Vertex>> {
    match kernelization(graph, k) {
        KernelResult::Failed => None,
        KernelResult::Success(instance) => {
            if let Some(mut fpt_cover) = fpt_algorithm_opt(instance.reduced_graph, instance.new_k) {
                fpt_cover.extend(instance.partial_cover);
                Some(fpt_cover)
            } 
            else {
                None
            }
        }
    }
}

// ALGORYTM SYMPLEX (PYTHON)
pub fn get_lp_relaxation(filename: &str) -> String {
    let output: std::process::Output = Command::new("python").arg("lp_solver.py").arg(filename).output().expect("Cannot run python script!");

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } 
    else {
        let err: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stderr);
        panic!("Python err: {}", err);
    }
}

pub fn find_vertex_cover_with_symplex_1(mut graph: Graph, mut k: usize, filename: &str) -> Option<Vec<Vertex>> {
    let output: String = get_lp_relaxation(filename);

    let mut v_0: Vec<Vertex> = Vec::new();
    let mut v_1_2: Vec<Vertex> = Vec::new();
    let mut v_1: Vec<Vertex> = Vec::new();
    let mut sum: f64 = 0.0;

    for line in output.lines() {
        let mut parts: std::str::SplitWhitespace<'_> = line.split_whitespace();

        let vertex: Vertex = parts.next().unwrap().parse().unwrap();
        let vertex_value: f64 = parts.next().unwrap().parse().unwrap();

        sum += vertex_value;

        if vertex_value < 1.0 / 2.0 {
            v_0.push(vertex);
        }
        else if vertex_value == 1.0 / 2.0 {
            v_1_2.push(vertex);
        }
        else {
            v_1.push(vertex);
        }
    }
    
    if (k as f64) < sum {
        return None;
    }

    k -= v_1.len();

    for u in v_0 {
        graph.remove_vertex(u);
    }
    for u in &v_1 {
        graph.remove_vertex(*u);
    }

    if let Some(mut fpt_cover) = fpt_algorithm(graph, k) {
        fpt_cover.extend(v_1);
        Some(fpt_cover)
    }
    else {
        None
    }
}

pub fn find_vertex_cover_with_symplex_2(mut graph: Graph, mut k: usize, filename: &str) -> Option<Vec<Vertex>> {
    let output: String = get_lp_relaxation(filename);

    let mut v_0: Vec<Vertex> = Vec::new();
    let mut v_1_2: Vec<Vertex> = Vec::new();
    let mut v_1: Vec<Vertex> = Vec::new();
    let mut sum: f64 = 0.0;

    for line in output.lines() {
        let mut parts: std::str::SplitWhitespace<'_> = line.split_whitespace();

        let vertex: Vertex = parts.next().unwrap().parse().unwrap();
        let vertex_value: f64 = parts.next().unwrap().parse().unwrap();

        sum += vertex_value;

        if vertex_value < 1.0 / 2.0 {
            v_0.push(vertex);
        }
        else if vertex_value == 1.0 / 2.0 {
            v_1_2.push(vertex);
        }
        else {
            v_1.push(vertex);
        }
    }
    
    if (k as f64) < sum {
        return None;
    }

    k -= v_1.len();

    for u in v_0 {
        graph.remove_vertex(u);
    }
    for u in &v_1 {
        graph.remove_vertex(*u);
    }

    if let Some(mut fpt_cover) = fpt_algorithm_opt(graph, k) {
        fpt_cover.extend(v_1);
        Some(fpt_cover)
    }
    else {
        None
    }
}

