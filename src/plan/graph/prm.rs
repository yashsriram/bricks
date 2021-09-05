use super::{Graph, Vertex};
use crate::plan::{State, StateSpace};
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
            state_space: state_space,
            graph: Graph {
                vertices: states
                    .into_iter()
                    .zip(adjacencies.into_iter())
                    .map(|(state, adjacencies)| Vertex { state, adjacencies })
                    .collect(),
            },
        }
    }

    pub fn add(&mut self, states: Vec<S::State>, edge_len: f32) -> Vec<usize> {
        let prev_graph_size = self.graph.vertices.len();
        for state in states.into_iter() {
            self.graph.vertices.push(Vertex {
                state: state,
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
        (prev_graph_size..self.graph.vertices.len()).collect()
    }
}
