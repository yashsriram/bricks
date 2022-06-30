use super::spaces::CuboidSpace;
use crate::graph::{Graph, Vertex};
use nalgebra::Point3;
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

pub struct PRM {
    pub graph: Graph,
}

impl PRM {
    pub fn with_num_samples(state_space: &CuboidSpace, num_samples: usize, edge_len: f32) -> Self {
        let mut rng = thread_rng();
        let state_samples: Vec<Point3<f32>> = (&mut rng)
            .sample_iter(Standard)
            .take(num_samples)
            .map(|(x, y, z): (f32, f32, f32)| {
                Point3::new(
                    x * state_space.size.x,
                    y * state_space.size.y,
                    z * state_space.size.z,
                )
            })
            .collect();
        let mut adjacencies = vec![HashSet::new(); state_samples.len()];
        for i in 0..(state_samples.len() - 1) {
            let s1 = &state_samples[i];
            for j in (i + 1)..state_samples.len() {
                let s2 = &state_samples[j];
                if (s1 - s2).norm() <= edge_len {
                    adjacencies[i].insert(j);
                    adjacencies[j].insert(i);
                }
            }
        }
        Self {
            graph: Graph {
                vertices: state_samples
                    .into_iter()
                    .zip(adjacencies.into_iter())
                    .map(|(state, adjacencies)| Vertex { state, adjacencies })
                    .collect(),
            },
        }
    }

    pub fn add<const N: usize>(&mut self, states: [Point3<f32>; N], edge_len: f32) -> [usize; N] {
        let prev_graph_size = self.graph.vertices.len();
        for state in IntoIterator::into_iter(states) {
            self.graph.vertices.push(Vertex {
                state,
                adjacencies: HashSet::new(),
            });
        }
        for i in (prev_graph_size..self.graph.vertices.len()).rev() {
            for j in 0..(i - 1) {
                if (self.graph.vertices[i].state - &self.graph.vertices[j].state).norm() <= edge_len
                {
                    self.graph.vertices[i].adjacencies.insert(j);
                    self.graph.vertices[j].adjacencies.insert(i);
                }
            }
        }
        let mut idxes = [0; N];
        for (i, idx) in (prev_graph_size..self.graph.vertices.len()).enumerate() {
            idxes[i] = idx;
        }
        idxes
    }
}
