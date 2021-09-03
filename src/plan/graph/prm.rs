use super::{Graph, Vertex};
use crate::plan::{State, StateSpace};
use std::collections::BTreeSet;

pub struct PRM<S: StateSpace> {
    pub state_space: S,
    pub graph: Graph<S>,
}

impl<S: StateSpace> PRM<S> {
    pub fn with_num_samples(state_space: S, num_samples: usize, edge_len: f32) -> Self {
        let states: Vec<S::State> = state_space.sample_batch(num_samples);
        let mut adjacencies = vec![BTreeSet::new(); states.len()];
        for i in 0..(states.len() - 1) {
            let v1 = &states[i];
            for j in (i + 1)..states.len() {
                let v2 = &states[j];
                if v1.dist(v2) <= edge_len {
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
}
