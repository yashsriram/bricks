use super::{State, StateSpace};
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::pipeline::PrimitiveTopology;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::fmt::Debug;

// pub trait VertexSearchState: Debug + Default + Ord + PartialOrd {
//     fn vertex_idx(&self) -> usize;
// }

// pub trait GraphSearchState {
//     type VertexSearchState: VertexSearchState;

//     fn start_with(idx: usize) -> Self::VertexSearchState;

//     fn consume_adjacencies(this: usize, neighbours: &Vec) -> Self::VertexSearchState;
// }

pub struct Graph<T: StateSpace> {
    vertices: Vec<T::State>,
    adjacencies: Vec<BTreeSet<usize>>,
}

// impl<T: StateSpace> Graph<T> {
//     pub fn bfs<G: GraphSearchState>(&self, start: usize, finish: usize) {
//         let mut fringe = BinaryHeap::with_capacity(self.vertices.len());
//         fringe.push(G::VertexSearchState::default());
//         // Local search state -> pass as arg and return
//         // Ord on Priority Queue
//         // Start on heap
//         // Need to have vertex id in search state
//         while let Some(curr) = fringe.pop() {
//             if curr.vertex_idx() == finish {
//                 break;
//             }
//             // Change state based on popping
//             for neighbours in self.adjacencies[curr].iter() {
//                 // If adding to fringe
//                 // Change state based on prev and curr vertex state
//                 // Need to have vertex id in search state
//                 // Add to fringe
//             }
//         }
//     }
// }

impl<T: StateSpace> From<Graph<T>> for Mesh {
    fn from(graph: Graph<T>) -> Mesh {
        let vertex_positions: Vec<[f32; 3]> = graph
            .vertices
            .iter()
            .map(|v| v.projection_to_3d())
            .collect();
        let vertex_colors = vec![[1.0, 1.0, 1.0, 0.5]; graph.vertices.len()];
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
