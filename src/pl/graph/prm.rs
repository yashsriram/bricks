use super::super::{State, StateSpace};
use super::{Graph, Vertex};
use std::collections::HashSet;

pub struct PRM<S: StateSpace> {
    pub state_space: S,
    pub graph: Graph<S>,
}

impl<S: StateSpace> PRM<S> {
    pub fn with_num_samples(state_space: S, num_samples: usize, edge_len: f32) -> Self {
        let states: Vec<S::State> = state_space.sample_batch(num_samples);
        let mut adjacencies = vec![HashSet::new(); states.len()];
        for i in 0..(states.len() - 1) {
            let s1 = &states[i];
            for j in (i + 1)..states.len() {
                let s2 = &states[j];
                if s1.dist(s2) <= edge_len {
                    adjacencies[i].insert(j);
                    adjacencies[j].insert(i);
                }
            }
        }
        Self {
            state_space,
            graph: Graph {
                vertices: states
                    .into_iter()
                    .zip(adjacencies.into_iter())
                    .map(|(state, adjacencies)| Vertex { state, adjacencies })
                    .collect(),
            },
        }
    }

    pub fn add<const N: usize>(&mut self, states: [S::State; N], edge_len: f32) -> [usize; N] {
        let prev_graph_size = self.graph.vertices.len();
        for state in IntoIterator::into_iter(states) {
            self.graph.vertices.push(Vertex {
                state,
                adjacencies: HashSet::new(),
            });
        }
        for i in (prev_graph_size..self.graph.vertices.len()).rev() {
            for j in 0..(i - 1) {
                if self.graph.vertices[i]
                    .state
                    .dist(&self.graph.vertices[j].state)
                    <= edge_len
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
