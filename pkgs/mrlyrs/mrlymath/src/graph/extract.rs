use super::models::Network;
use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::Tensor;
use std::collections::HashSet;

fn center(coord: &[usize]) -> Vec<f64> {
    coord.iter().rev().map(|&c| c as f64 + 0.5).collect()
}

fn coords_of(grid: &Tensor) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    for flat in 0..grid.size() {
        if grid.bytes()[flat] != 0 {
            let mut rem = flat;
            let mut multi = Vec::with_capacity(grid.shape.len());
            for axis in 0..grid.shape.len() {
                let stride: usize = grid.shape[(axis + 1)..].iter().product();
                multi.push(rem / stride);
                rem %= stride;
            }
            out.push(multi);
        }
    }
    out
}

pub fn core_graph(grid: &Tensor) -> Result<Network> {
    let dim = grid.shape.len();
    if dim != 2 && dim != 3 {
        return value_error("core_graph expects a 2D or 3D cell.");
    }
    let coords = coords_of(grid);
    let mut index_of = vec![usize::MAX; grid.size()];
    for (i, coord) in coords.iter().enumerate() {
        index_of[grid.index(coord)] = i;
    }
    let mut network = Network::new(dim);
    for coord in &coords {
        network.add_node(center(coord))?;
    }
    for coord in &coords {
        for axis in 0..dim {
            if coord[axis] + 1 >= grid.shape[axis] {
                continue;
            }
            let mut neighbor = coord.clone();
            neighbor[axis] += 1;
            if grid.get(&neighbor) != 0 {
                network.add_branch(
                    index_of[grid.index(coord)],
                    index_of[grid.index(&neighbor)],
                    1.0,
                )?;
            }
        }
    }
    Ok(network)
}

pub fn tunnel_graph(grid: &Tensor) -> Result<Network> {
    core_graph(&grid.invert())
}

pub fn edge_graph(grid: &Tensor) -> Result<Network> {
    let dim = grid.shape.len();
    if dim != 2 && dim != 3 {
        return value_error("edge_graph expects a 2D or 3D cell.");
    }
    let corner_shape: Vec<usize> = grid.shape.iter().map(|&s| s + 1).collect();
    let corner_index_dims = corner_shape.clone();
    let flat_corner = |coord: &[usize]| -> usize {
        let mut out = 0;
        for (axis, &c) in coord.iter().enumerate() {
            out = out * corner_index_dims[axis] + c;
        }
        out
    };
    let coords = coords_of(grid);
    let mut used: HashSet<Vec<usize>> = HashSet::new();
    for coord in &coords {
        for offset in 0..(1usize << dim) {
            let corner: Vec<usize> = (0..dim)
                .map(|axis| coord[axis] + ((offset >> (dim - 1 - axis)) & 1))
                .collect();
            used.insert(corner);
        }
    }
    let mut ordered: Vec<Vec<usize>> = used.into_iter().collect();
    ordered.sort();
    let mut corner_node = vec![usize::MAX; corner_shape.iter().product()];
    let mut network = Network::new(dim);
    for corner in &ordered {
        let position: Vec<f64> = corner.iter().rev().map(|&c| c as f64).collect();
        corner_node[flat_corner(corner)] = network.add_node(position)?;
    }
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    for coord in &coords {
        for (a, b) in cell_edges(coord, dim) {
            let ia = corner_node[flat_corner(&a)];
            let ib = corner_node[flat_corner(&b)];
            let key = (ia.min(ib), ia.max(ib));
            if seen.insert(key) {
                network.add_branch(ia, ib, 1.0)?;
            }
        }
    }
    Ok(network)
}

fn cell_edges(coord: &[usize], dim: usize) -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut pairs = Vec::new();
    for fixed_axis in 0..dim {
        for combo in 0..(1usize << (dim - 1)) {
            let mut lo = Vec::with_capacity(dim);
            let mut hi = Vec::with_capacity(dim);
            let mut bit = 0;
            for (axis, &c) in coord.iter().enumerate() {
                if axis == fixed_axis {
                    lo.push(c);
                    hi.push(c + 1);
                } else {
                    let v = (combo >> bit) & 1;
                    bit += 1;
                    lo.push(c + v);
                    hi.push(c + v);
                }
            }
            pairs.push((lo, hi));
        }
    }
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::atoms;
    #[test]
    fn single_square_graphs() {
        let g = atoms::ones_2d(1);
        let core = core_graph(&g).unwrap();
        assert_eq!(core.nodes.len(), 1);
        assert_eq!(core.branches.len(), 0);
        let edges = edge_graph(&g).unwrap();
        assert_eq!(edges.nodes.len(), 4);
        assert_eq!(edges.branches.len(), 4);
    }
    #[test]
    fn carpet_core_graph_counts() {
        let g = atoms::carpet_2d(3);
        let core = core_graph(&g).unwrap();
        assert_eq!(core.nodes.len(), 8);
        assert_eq!(core.branches.len(), 8);
        let tunnels = tunnel_graph(&g).unwrap();
        assert_eq!(tunnels.nodes.len(), 1);
    }
    #[test]
    fn cube_edge_graph() {
        let g = atoms::ones_3d(1);
        let edges = edge_graph(&g).unwrap();
        assert_eq!(edges.nodes.len(), 8);
        assert_eq!(edges.branches.len(), 12);
    }
}
