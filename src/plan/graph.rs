use super::{State, StateSpace};
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::pipeline::PrimitiveTopology;
use std::collections::BTreeSet;

pub struct Graph<T: StateSpace> {
    vertices: Vec<T::State>,
    adjacencies: Vec<BTreeSet<usize>>,
}

impl<T: StateSpace> Graph<T> {}

impl<T: StateSpace> From<Graph<T>> for Mesh {
    fn from(graph: Graph<T>) -> Mesh {
        let vertex_positions: Vec<[f32; 3]> = graph
            .vertices
            .iter()
            .map(|v| v.projection_to_3d())
            .collect();
        let vertex_colors = vec![[1.0, 1.0, 1.0]; graph.vertices.len()];
        let lines: Vec<u32> = graph
            .adjacencies
            .iter()
            .enumerate()
            .map(|(v_i, v_adjs)| {
                v_adjs
                    .iter()
                    .map(move |&neighbour| vec![v_i as u32, neighbour as u32])
                    .flatten()
            })
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

pub mod prm {
    use crate::plan::graph::Graph;
    use crate::plan::{State, StateSpace};
    use std::collections::BTreeSet;

    pub struct PRM<T: StateSpace> {
        pub state_space: T,
        pub graph: Graph<T>,
    }

    impl<T: StateSpace> PRM<T> {
        pub fn with_num_samples(state_space: T, num_samples: usize, edge_len: f32) -> Self {
            let vertices: Vec<T::State> = state_space.sample_batch(num_samples);
            let mut adjacencies = vec![BTreeSet::new(); vertices.len()];
            for i in 0..(vertices.len() - 1) {
                let v1 = &vertices[i];
                for j in (i + 1)..vertices.len() {
                    let v2 = &vertices[j];
                    if v1.dist(v2) <= edge_len {
                        adjacencies[i].insert(j);
                        adjacencies[j].insert(i);
                    }
                }
            }
            Self {
                state_space: state_space,
                graph: Graph {
                    vertices: vertices,
                    adjacencies: adjacencies,
                },
            }
        }
    }
}
