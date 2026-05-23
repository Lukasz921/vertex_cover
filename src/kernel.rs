use std::collections::*;
use crate::graph::*;
use good_lp::*;

pub fn kernelization(graph: &mut Graph, vertex_with_one: Option<Vertex>) -> (Vec<Vertex>, Count, Count, Count, Count) {
    let mut variables: ProblemVariables = ProblemVariables::new();
    
    let mut vars: HashMap<Vertex, Variable> = HashMap::new();
    let mut target: Expression = Expression::from(0.0);

    for &vertex in graph.neighborhood.keys() {
        let var: Variable = variables.add(variable().min(0.0).max(1.0));
        vars.insert(vertex, var);
        target += var;
    }

    let mut model: solvers::highs::HighsProblem = variables.minimise(target).using(default_solver);
    
    for &vertex in graph.neighborhood.keys() {
        let neighbors: Vec<Vertex> = graph.neighbors(vertex);
        for neighbor in neighbors {
            if vertex < neighbor {
                model.add_constraint((vars[&vertex] + vars[&neighbor]) >> 1.0);
            }
        }
    }

    if let Some(vertex) = vertex_with_one && let Some(&var) = vars.get(&vertex) {
        model.add_constraint(var >> 1.0);
    }

    let mut partial_cover: Vec<Vertex> = Vec::new();
    let mut to_remove: Vec<Vertex> = Vec::new();

    if let Ok(solution) = model.solve() {
        for (&vertex, &var) in &vars {
            let value: f64 = solution.value(var);
            if value > 0.5 {
                partial_cover.push(vertex);
            }
            if value != 0.5 {
                to_remove.push(vertex);
            }
        }
    };

    let n: Count = graph.size();
    let v12_count: Count = n - to_remove.len();
    let v1_count: Count = partial_cover.len();
    let v0_count: Count = n - v12_count - v1_count;
    
    for vertex in to_remove {
        graph.remove_vertex(vertex);
    }
    
    (partial_cover, v0_count, v12_count, v1_count, n)
}