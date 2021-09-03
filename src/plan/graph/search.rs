pub mod ripple {
    use super::super::Graph;
    use crate::plan::StateSpace;

    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::marker::PhantomData;

    pub trait CostPriority {
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

    pub trait VertexSearchState<S: StateSpace>:
        Debug + Clone + CostPriority + Propagate<S>
    {
    }

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
    pub struct RippleSearch<S: StateSpace, F: VertexSearchState<S>> {
        state_space: PhantomData<S>,
        start_idx: usize,
        finish_idx: usize,
        max_idx: usize,
        vertex_parent_map: HashMap<usize, Option<usize>>,
        ripple: HashMap<usize, F>,
    }

    impl<S: StateSpace, F: VertexSearchState<S>> RippleSearch<S, F> {
        pub fn try_search(graph: &Graph<S>, start_idx: usize, finish_idx: usize) -> Self {
            Self::try_search_with_alloc(graph, start_idx, finish_idx, 1.0)
        }

        pub fn try_search_with_alloc(
            graph: &Graph<S>,
            start_idx: usize,
            finish_idx: usize,
            initial_alloc_frac: f32,
        ) -> Self {
            assert!(start_idx < graph.vertices.len());
            assert!(finish_idx < graph.vertices.len());
            assert!(initial_alloc_frac >= 0.0);
            let collec_alloc_size = (graph.vertices.len() as f32 * initial_alloc_frac) as usize;
            let mut vertex_parent_map = HashMap::with_capacity(collec_alloc_size);
            vertex_parent_map.insert(start_idx, None);
            let mut ripple = HashMap::with_capacity(collec_alloc_size);
            ripple.insert(
                start_idx,
                F::as_start(start_idx, &graph.vertices[start_idx].state),
            );
            let mut fringe = BinaryHeap::with_capacity(collec_alloc_size);
            fringe.push(IndexedVertexSearchState {
                idx: start_idx,
                vertex_search_state: ripple[&start_idx].clone(),
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
                    if let None = ripple.get(&adj_idx) {
                        vertex_parent_map.insert(adj_idx, Some(curr_idx));
                        ripple.insert(
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
                            vertex_search_state: ripple[&adj_idx].clone(),
                            state_space: PhantomData,
                        });
                    }
                }
            }
            RippleSearch {
                state_space: PhantomData,
                start_idx: start_idx,
                finish_idx: finish_idx,
                max_idx: graph.vertices.len() - 1,
                vertex_parent_map: vertex_parent_map,
                ripple: ripple,
            }
        }

        pub fn start_idx(&self) -> usize {
            self.start_idx
        }

        pub fn finish_idx(&self) -> usize {
            self.finish_idx
        }

        pub fn max_idx(&self) -> usize {
            self.max_idx
        }

        pub fn path_to_finish(&self) -> Option<Vec<usize>> {
            self.path_to(self.finish_idx)
        }

        pub fn path_to(&self, goal_idx: usize) -> Option<Vec<usize>> {
            assert!(goal_idx <= self.max_idx);
            if goal_idx == self.start_idx {
                return Some(vec![self.start_idx]);
            }
            let mut vertex_idx = goal_idx;
            let mut path = vec![vertex_idx];
            while let Some(&Some(parent_idx)) = self.vertex_parent_map.get(&vertex_idx) {
                path.push(parent_idx);
                vertex_idx = parent_idx;
            }
            let path: Vec<usize> = path.into_iter().rev().collect();
            match path.len() {
                1 => None,
                _ => Some(path),
            }
        }
    }
}
