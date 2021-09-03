use super::{State, StateSpace};
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::pipeline::PrimitiveTopology;
use std::collections::BTreeSet;

pub mod prm;
pub mod search;

pub struct Vertex<S: StateSpace> {
    state: S::State,
    adjacencies: BTreeSet<usize>,
}

pub struct Graph<S: StateSpace> {
    vertices: Vec<Vertex<S>>,
}

impl<S: StateSpace> Graph<S> {}

impl<S: StateSpace> From<Graph<S>> for Mesh {
    fn from(graph: Graph<S>) -> Mesh {
        let vertex_positions: Vec<[f32; 3]> = graph
            .vertices
            .iter()
            .map(|Vertex { state, .. }| state.projection_to_3d())
            .collect();
        let vertex_colors = vec![[1.0, 1.0, 1.0, 0.5]; graph.vertices.len()];
        let lines: Vec<u32> = graph
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
        let indices = Indices::U32(lines);

        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertex_positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);
        mesh.set_indices(Some(indices));
        mesh
    }
}
