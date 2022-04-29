use bevy::render::{
    mesh::{Indices, Mesh},
    pipeline::PrimitiveTopology,
};
use nalgebra::Point3;
use std::collections::HashSet;

pub mod path;
pub mod prm;
pub mod search;

#[derive(Debug)]
pub struct Vertex {
    pub(crate) state: Point3<f32>,
    pub(crate) adjacencies: HashSet<usize>,
}

#[derive(Debug)]
pub struct Graph {
    pub(crate) vertices: Vec<Vertex>,
}

impl Graph {
    pub fn num_edges(&self) -> usize {
        self.vertices
            .iter()
            .map(
                |Vertex {
                     state: _,
                     adjacencies,
                 }| adjacencies.len(),
            )
            .sum()
    }
}

impl From<&Graph> for Mesh {
    fn from(graph: &Graph) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        let positions: Vec<[f32; 3]> = graph
            .vertices
            .iter()
            .map(|Vertex { state, .. }| [state.x, state.y, state.z])
            .collect();
        let indices: Vec<u32> = graph
            .vertices
            .iter()
            .enumerate()
            .map(
                |(
                    v_i,
                    Vertex {
                        state: _,
                        adjacencies: v_adjs,
                    },
                )| {
                    v_adjs
                        .iter()
                        .map(move |&adj| vec![v_i as u32, adj as u32])
                        .flatten()
                },
            )
            .into_iter()
            .flatten()
            .collect();
        let indices = Indices::U32(indices);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_indices(Some(indices));
        let colors = vec![[1.0, 1.0, 1.0, 0.1]; graph.vertices.len()];
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh
    }
}
