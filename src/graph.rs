use std::collections::*;

pub type Vertex = usize;
pub type Count = usize;

#[derive(Clone)]
pub struct Graph {
    pub neighborhood: HashMap<Vertex, HashSet<Vertex>>
}

impl Graph {
    // NEW INSTANCE OF THE GRAPH WITH SIZE N
    pub fn new(n: Count) -> Graph {
        let mut neighborhood: HashMap<Vertex, HashSet<Vertex>> = HashMap::with_capacity(n);
        for vertex in 0..n {
            neighborhood.insert(vertex, HashSet::new());
        }
        Graph { neighborhood }
    }

    // ADDING EDGE FROM VERTEX_1 TO VERTEX_2
    pub fn add_edge(&mut self, vertex_1: Vertex, vertex_2: Vertex) {
        if let Some(neighbors_1) = self.neighborhood.get_mut(&vertex_1) {
            neighbors_1.insert(vertex_2);
        };
        if let Some(neighbors_2) = self.neighborhood.get_mut(&vertex_2) {
            neighbors_2.insert(vertex_1);
        };
    }

    // get |V|
    pub fn size(&self) -> Count {
        self.neighborhood.len()
    }

    // IS |V| == 0
    pub fn is_empty(&self) -> bool {
        self.neighborhood.len() == 0
    }

    // DEGREE OF GIVEN VERTEX
    pub fn degree(&self, vertex: Vertex) -> Count {
        if let Some(neighbors) = self.neighborhood.get(&vertex) {
            return neighbors.len();
        };
        0
    }

    // MAXIMUM DEGREE IN GRAPH
    pub fn max_degree(&mut self) -> Vertex {
        let mut max_degree: Count = 0;
        let mut max_vertex: Vertex = 0;
        for vertex in self.neighborhood.keys() {
            if max_degree < self.degree(*vertex) {
                max_degree = self.degree(*vertex);
                max_vertex = *vertex;
            }
        }
        max_vertex
    }

    // GET NEIGHBORS OF GIVEN VERTEX
    pub fn neighbors(&self, vertex: Vertex) -> Vec<Vertex> {
        let mut vertex_neighbors: Vec<Vertex> = Vec::new();
        if let Some(neighbors) = self.neighborhood.get(&vertex) {
            for neighbor in neighbors {
                vertex_neighbors.push(*neighbor);
            }
        }; 
        vertex_neighbors
    }

    // REMOVE VERTEX FROM GRAPH
    pub fn remove_vertex(&mut self, vertex: Vertex) -> Vertex {
        let vertex_neighbors: Vec<Vertex> = self.neighbors(vertex);
        for neighbor in vertex_neighbors {
            if let Some(neighbors) = self.neighborhood.get_mut(&neighbor) {
                neighbors.remove(&vertex);
            }
        }
        self.neighborhood.remove(&vertex);
        vertex
    }
}