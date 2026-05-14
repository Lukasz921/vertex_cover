use crate::graphs::graph::*;

pub enum KernelResult {
    Success(KernelizedInstance),
    Failed
}

pub struct KernelizedInstance {
    pub reduced_graph: Graph,
    pub new_k: usize,
    pub partial_cover: Vec<Vertex>
}

pub fn kernelization(mut graph: Graph, mut k: usize) -> KernelResult {
    let mut rule_a: bool = true;
    let mut rule_b: bool = true;

    let mut partial_cover: Vec<Vertex> = Vec::new();

    while rule_a || rule_b {
        rule_a = false;
        rule_b = false;

        if let Some(u) = graph.get_isolated_vertex() {
            graph.remove_vertex(u);
            rule_a = true;
        };

        if let Some(u) = graph.get_kernel_vertex(k) {        
            if k == 0 {
                return KernelResult::Failed;
            }

            graph.remove_vertex(u);
            partial_cover.push(u);
            
            k -= 1;

            rule_b = true;
        };
    }

    if k * k + k < graph.active_vertex_count() || k * k < graph.edges_count() {
        return KernelResult::Failed;
    }

    let kernelized_instance: KernelizedInstance = KernelizedInstance { reduced_graph: graph, new_k: k, partial_cover };
    KernelResult::Success(kernelized_instance)
}