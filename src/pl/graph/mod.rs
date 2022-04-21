use super::StateSpace;
use std::collections::HashSet;

pub mod path;
pub mod prm;
pub mod search;

#[derive(Debug)]
pub struct Vertex<S: StateSpace> {
    pub(crate) state: S::State,
    pub(crate) adjacencies: HashSet<usize>,
}

#[derive(Debug)]
pub struct Graph<S: StateSpace> {
    pub(crate) vertices: Vec<Vertex<S>>,
}

impl<S: StateSpace> Graph<S> {
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

use super::State;
use crate::vz::bevy::render::{
    mesh::{Indices, Mesh},
    pipeline::PrimitiveTopology,
};

impl<SS: StateSpace> From<&Graph<SS>> for Mesh {
    fn from(graph: &Graph<SS>) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        let positions: Vec<[f32; 3]> = graph
            .vertices
            .iter()
            .map(|Vertex { state, .. }| state.project_to_3d())
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
