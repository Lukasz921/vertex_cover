use std::collections::{HashSet};

pub type Vertex = usize;

#[derive(Clone)]
pub struct Graph {
    pub active_vertices: Vec<bool>,
    pub neighbors: Vec<HashSet<Vertex>>
}

impl Graph {
    // NEW GRAPH
    pub fn new(n: usize) -> Graph {
        let active_vertices: Vec<bool> = vec![true; n];
        let mut neighbors: Vec<HashSet<Vertex>> = Vec::new();
        for _ in 0..n {
            neighbors.push(HashSet::new());
        }
        Graph { active_vertices, neighbors }
    }

    // OVERALL VERTEX COUNT, ACTIVE VERTEX COUNT, EDGE COUNT
    pub fn vertex_count(&self) -> usize {
        self.active_vertices.len()
    }
    pub fn active_vertex_count(&self) -> usize {
        self.active_vertices.iter().filter(|&&b| b).count()
    }
    pub fn edges_count(&self) -> usize {
        let mut counter: usize = 0;
        for u in 0..self.active_vertices.len() {
            counter += self.degree(u);
        }     
        counter / 2
    }

    // IS G EMPTY (FTP)
    pub fn is_empty(&self) -> bool {
        self.edges_count() == 0
    }

    // IS VERTEX ACTIVE, IS/ARE VERTEX/VERTICES OK
    pub fn is_vertex_active(&self, u: Vertex) -> bool {
        self.active_vertices[u]
    }
    pub fn is_vertex_ok(&self, u: Vertex) -> bool {
        u < self.vertex_count() && self.active_vertices[u]
    }
    pub fn are_vertices_ok(&self, u: Vertex, v: Vertex) -> bool {
        self.is_vertex_ok(u) && self.is_vertex_ok(v)
    }

    // ADD EDGE
    pub fn add_edge(&mut self, u: Vertex, v: Vertex) -> bool {
        if self.are_vertices_ok(u, v) {
            return self.neighbors[u].insert(v) && self.neighbors[v].insert(u);
        }
        false
    }

    // REMOVE EDGE
    pub fn remove_vertex(&mut self, u: Vertex) -> bool {
        if self.is_vertex_ok(u) {
            let neighbors: HashSet<usize> = self.neighbors[u].clone();

            for v in neighbors {
                self.neighbors[v].remove(&u);
            }

            self.neighbors[u].clear();
            self.active_vertices[u] = false;

            return true;
        }
        false
    }
    
    // DEGREE
    pub fn degree(&self, u: Vertex) -> usize {
        if self.is_vertex_ok(u) {
            return self.neighbors[u].len();
        }
        0
    }

    // CURRENT MAX DEGREE
    pub fn max_degree_vertex(&self) -> Option<Vertex> {
        if self.is_empty() {
            return None;
        }

        let mut idx_max: usize = 0;
        for (idx, is_active) in self.active_vertices.iter().enumerate() {
            if *is_active && self.degree(idx_max) < self.degree(idx) {
                idx_max = idx;
            }
        }
        Some(idx_max)
    }

    // GET RANDOM EDGE
    pub fn get_arbitrary_edge(&self) -> Option<(Vertex, Vertex)> {
        for u in 0..self.neighbors.len() {
            if let Some(v) = self.neighbors[u].iter().next() {
                return Some((u, *v));
            }
        }
        None
    }

    // KERNELIZATION A
    pub fn get_isolated_vertex(&self) -> Option<Vertex> {
        for (u, is_active) in self.active_vertices.iter().enumerate() {
            if *is_active && self.neighbors[u].is_empty() {
                return Some(u);
            }
        }
        None
    }

    // GET NEIGHBORS OF GIVEN VERTEX
    pub fn get_neighbors(&self, u: Vertex) -> Vec<Vertex> {
        let mut neighbors: Vec<Vertex> = Vec::new();
        for neighbor in &self.neighbors[u] {
            neighbors.push(*neighbor);
        }
        neighbors
    }

    // KERNALIZATION B - GET VERTEX OF DEGREE > K
    pub fn get_kernel_vertex(&self, k: usize) -> Option<Vertex> {
        self.active_vertices.iter().enumerate().filter(|&(_, &is_active)| is_active).map(|(u, _)| u).find(|&u| k < self.degree(u))
    }
}