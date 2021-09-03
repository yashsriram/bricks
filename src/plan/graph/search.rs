use super::Graph;
use crate::plan::StateSpace;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait Cost {
    fn cost(&self) -> f32;
}

pub trait Propagate<S: StateSpace> {
    fn as_start(start_vertex_idx: usize, start_state: &S::State) -> Self;

    fn as_adj(
        prev_vertex_idx: usize,
        prev_state: &S::State,
        my_vertex_idx: usize,
        my_state: &S::State,
        prev_search_state: &Self,
    ) -> Self;
}

pub trait VertexSearchState<S: StateSpace>: Debug + Clone + Cost + Propagate<S> {}

struct IndexedVertexSearchState<S: StateSpace, F: VertexSearchState<S>> {
    state_space: PhantomData<S>,
    idx: usize,
    vertex_search_state: F,
}

impl<S: StateSpace, F: VertexSearchState<S>> PartialEq for IndexedVertexSearchState<S, F> {
    fn eq(&self, other: &Self) -> bool {
        self.vertex_search_state.cost() == other.vertex_search_state.cost()
    }
}

impl<S: StateSpace, F: VertexSearchState<S>> Eq for IndexedVertexSearchState<S, F> {}

impl<S: StateSpace, F: VertexSearchState<S>> Ord for IndexedVertexSearchState<S, F> {
    fn cmp(&self, other: &Self) -> Ordering {
        let other_cost = other.vertex_search_state.cost();
        let self_cost = self.vertex_search_state.cost();
        if other_cost == f32::NAN && self_cost == f32::NAN {
            Ordering::Equal
        } else if other_cost != f32::NAN && self_cost == f32::NAN {
            Ordering::Less
        } else if other_cost == f32::NAN && self_cost != f32::NAN {
            Ordering::Greater
        } else {
            other_cost.partial_cmp(&self_cost).unwrap()
        }
    }
}

impl<S: StateSpace, F: VertexSearchState<S>> PartialOrd for IndexedVertexSearchState<S, F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct FringeBasedSearch<S: StateSpace, F: VertexSearchState<S>> {
    state_space: PhantomData<S>,
    pub vertex_search_states: HashMap<usize, F>,
}

impl<S: StateSpace, F: VertexSearchState<S>> FringeBasedSearch<S, F> {
    pub fn search(graph: &Graph<S>, start_idx: usize, finish_idx: usize) -> Self {
        assert!(start_idx < graph.vertices.len());
        assert!(finish_idx < graph.vertices.len());
        let mut vertex_search_states = HashMap::with_capacity(graph.vertices.len());
        vertex_search_states.insert(
            start_idx,
            F::as_start(start_idx, &graph.vertices[start_idx].state),
        );
        let mut fringe = BinaryHeap::with_capacity(graph.vertices.len());
        fringe.push(IndexedVertexSearchState {
            idx: start_idx,
            vertex_search_state: vertex_search_states[&start_idx].clone(),
            state_space: PhantomData,
        });
        while let Some(IndexedVertexSearchState {
            idx: curr_idx,
            vertex_search_state: curr_search_state,
            ..
        }) = fringe.pop()
        {
            if curr_idx == finish_idx {
                break;
            }
            for &adj_idx in graph.vertices[curr_idx].adjacencies.iter() {
                let curr_vertex = &graph.vertices[curr_idx];
                let adj_vertex = &graph.vertices[adj_idx];
                if let None = vertex_search_states.get(&adj_idx) {
                    vertex_search_states.insert(
                        adj_idx,
                        F::as_adj(
                            curr_idx,
                            &curr_vertex.state,
                            adj_idx,
                            &adj_vertex.state,
                            &curr_search_state,
                        ),
                    );
                    fringe.push(IndexedVertexSearchState {
                        idx: adj_idx,
                        vertex_search_state: vertex_search_states[&adj_idx].clone(),
                        state_space: PhantomData,
                    });
                }
            }
        }
        FringeBasedSearch {
            vertex_search_states: vertex_search_states,
            state_space: PhantomData,
        }
    }
}
