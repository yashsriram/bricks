pub mod ripple {
    use super::super::Graph;
    use crate::plan::StateSpace;

    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::marker::PhantomData;

    pub trait Propagate<S: StateSpace>: Debug + Sized {
        fn as_start(start_vertex_idx: usize, start_state: &S::State) -> (f32, Self);

        fn as_adj(
            prev_vertex_idx: usize,
            prev_state: &S::State,
            my_vertex_idx: usize,
            my_state: &S::State,
            prev_search_state: &Self,
        ) -> (f32, Self);
    }

    struct CostPriority {
        vertex_idx: usize,
        vertex_cost: f32,
    }

    impl PartialEq for CostPriority {
        fn eq(&self, other: &Self) -> bool {
            self.vertex_cost == other.vertex_cost
        }
    }

    impl Eq for CostPriority {}

    impl Ord for CostPriority {
        fn cmp(&self, other: &Self) -> Ordering {
            let other_cost = other.vertex_cost;
            let self_cost = self.vertex_cost;
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

    impl PartialOrd for CostPriority {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug)]
    pub struct RippleSearch<S: StateSpace, F: Propagate<S>> {
        state_space: PhantomData<S>,
        start_idx: usize,
        stop_idx: usize,
        max_idx: usize,
        vertex_parent_map: HashMap<usize, Option<usize>>,
        ripple: HashMap<usize, F>,
    }

    impl<S: StateSpace, F: Propagate<S>> RippleSearch<S, F> {
        pub fn try_search(graph: &Graph<S>, start_idx: usize, stop_idx: usize) -> Self {
            Self::try_search_with_alloc(graph, start_idx, stop_idx, 1.0)
        }

        pub fn try_search_with_alloc(
            graph: &Graph<S>,
            start_idx: usize,
            stop_idx: usize,
            initial_alloc_frac: f32,
        ) -> Self {
            assert!(start_idx < graph.vertices.len());
            assert!(stop_idx < graph.vertices.len());
            assert!(initial_alloc_frac >= 0.0);
            let (start_cost, start_search_state) =
                F::as_start(start_idx, &graph.vertices[start_idx].state);
            let collec_alloc_size = (graph.vertices.len() as f32 * initial_alloc_frac) as usize;
            let mut vertex_parent_map = HashMap::with_capacity(collec_alloc_size);
            vertex_parent_map.insert(start_idx, None);
            let mut ripple = HashMap::with_capacity(collec_alloc_size);
            ripple.insert(start_idx, start_search_state);
            let mut fringe = BinaryHeap::with_capacity(collec_alloc_size);
            fringe.push(CostPriority {
                vertex_idx: start_idx,
                vertex_cost: start_cost,
            });
            while let Some(CostPriority {
                vertex_idx: curr_idx,
                ..
            }) = fringe.pop()
            {
                if curr_idx == stop_idx {
                    break;
                }
                for &adj_idx in graph.vertices[curr_idx].adjacencies.iter() {
                    if let None = ripple.get(&adj_idx) {
                        let (adj_cost, adj_search_state) = F::as_adj(
                            curr_idx,
                            &graph.vertices[curr_idx].state,
                            adj_idx,
                            &graph.vertices[adj_idx].state,
                            &ripple[&curr_idx],
                        );
                        vertex_parent_map.insert(adj_idx, Some(curr_idx));
                        ripple.insert(adj_idx, adj_search_state);
                        fringe.push(CostPriority {
                            vertex_idx: adj_idx,
                            vertex_cost: adj_cost,
                        });
                    }
                }
            }
            RippleSearch {
                state_space: PhantomData,
                start_idx: start_idx,
                stop_idx: stop_idx,
                max_idx: graph.vertices.len() - 1,
                vertex_parent_map: vertex_parent_map,
                ripple: ripple,
            }
        }

        pub fn start_idx(&self) -> usize {
            self.start_idx
        }

        pub fn stop_idx(&self) -> usize {
            self.stop_idx
        }

        pub fn max_idx(&self) -> usize {
            self.max_idx
        }

        pub fn path_to_stop(&self) -> Option<Vec<usize>> {
            self.path_to(self.stop_idx)
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
