use crate::kernel::*;
use crate::graph::*;

pub fn fpt(mut g_a: Graph, mut k: usize) -> Option<Vec<Vertex>> {

    let mut local_cover: Vec<Vertex> = Vec::new();
    let mut kernelization_active: bool = true;

    while kernelization_active {
        kernelization_active = false;
        
        let partial_cover: Vec<Vertex> = kernelization(&mut g_a, None);

        if !partial_cover.is_empty() {

            if partial_cover.len() > k {
                return None;
            }
            k -= partial_cover.len();
            local_cover.extend(partial_cover);
            kernelization_active = true;
        }
    }

    if g_a.is_empty() {
        return Some(local_cover);
    }
    if k == 0 {
        return None;
    }
    
    let lb = g_a.size().div_ceil(2);
    if k < lb { // bo matma udowodnila, ze wierzcholkow w pokryciu ze zbioru v 1/2 jest przynajmniej polowa z nich
        return None; 
    }

    let max_v: Vertex = g_a.max_degree();
    let neighbors: Vec<Vertex> = g_a.neighbors(max_v);
    let deg: Count = neighbors.len();

    if deg == 0 {
        return Some(local_cover);
    }

    let mut g_b: Graph = g_a.clone();
    g_b.remove_vertex(max_v);
    
    if let Some(mut cover_a) = fpt(g_b, k - 1) {
        cover_a.push(max_v);
        cover_a.extend(local_cover);
        return Some(cover_a);
    }

    if k >= deg {
        g_a.remove_vertex(max_v);
        for &u in &neighbors {
            g_a.remove_vertex(u);
        }
        
        if let Some(mut cover_b) = fpt(g_a, k - deg) {
            cover_b.extend(neighbors);
            cover_b.extend(local_cover);
            return Some(cover_b);
        }
    }

    None
}