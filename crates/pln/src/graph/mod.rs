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
