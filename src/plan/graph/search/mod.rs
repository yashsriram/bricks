pub mod spanning {
    use super::super::Graph;
    use crate::plan::StateSpace;
    use ordered_float::OrderedFloat;
    use std::cmp::Ordering;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::fmt::Debug;

    pub trait Propagation<SS: StateSpace>: Debug + Sized {
        fn as_start(
            my_vertex_state: &SS::State,
            stop_vertex_state: &SS::State,
        ) -> (OrderedFloat<f32>, Self);

        fn as_adj(
            prev_vertex_state: &SS::State,
            my_vertex_state: &SS::State,
            stop_vertex_state: &SS::State,
            parent: &Self,
        ) -> (OrderedFloat<f32>, Self);
    }

    pub mod propagations {
        use super::Propagation;
        use crate::plan::{State, StateSpace};
        use ordered_float::OrderedFloat;

        #[derive(Debug)]
        pub struct DFSLike {
            order: isize,
        }

        impl<SS: StateSpace> Propagation<SS> for DFSLike {
            fn as_start(_: &SS::State, _: &SS::State) -> (OrderedFloat<f32>, Self) {
                let me = Self { order: -0 };
                (OrderedFloat(me.order as f32), me)
            }

            fn as_adj(
                _: &SS::State,
                _: &SS::State,
                _: &SS::State,
                parent: &Self,
            ) -> (OrderedFloat<f32>, Self) {
                let me = Self {
                    order: parent.order - 1,
                };
                (OrderedFloat(me.order as f32), me)
            }
        }

        #[derive(Debug)]
        pub struct BFSLike {
            jumps_from_start: usize,
        }

        impl<SS: StateSpace> Propagation<SS> for BFSLike {
            fn as_start(_: &SS::State, _: &SS::State) -> (OrderedFloat<f32>, Self) {
                let me = Self {
                    jumps_from_start: 0,
                };
                (OrderedFloat(me.jumps_from_start as f32), me)
            }

            fn as_adj(
                _: &SS::State,
                _: &SS::State,
                _: &SS::State,
                parent: &Self,
            ) -> (OrderedFloat<f32>, Self) {
                let me = Self {
                    jumps_from_start: parent.jumps_from_start + 1,
                };
                (OrderedFloat(me.jumps_from_start as f32), me)
            }
        }

        #[derive(Debug)]
        pub struct UCSLike {
            dist_from_start: f32,
        }

        impl<SS: StateSpace> Propagation<SS> for UCSLike {
            fn as_start(_: &SS::State, _: &SS::State) -> (OrderedFloat<f32>, Self) {
                let me = Self {
                    dist_from_start: 0.0,
                };
                (OrderedFloat(me.dist_from_start), me)
            }

            fn as_adj(
                prev_vertex_state: &SS::State,
                my_vertex_state: &SS::State,
                _: &SS::State,
                parent: &Self,
            ) -> (OrderedFloat<f32>, Self) {
                let me = Self {
                    dist_from_start: parent.dist_from_start
                        + prev_vertex_state.dist(&my_vertex_state),
                };
                (OrderedFloat(me.dist_from_start), me)
            }
        }

        #[derive(Debug)]
        pub struct AStarLike {
            dist_from_start: f32,
        }

        impl<SS: StateSpace> Propagation<SS> for AStarLike {
            fn as_start(
                my_vertex_state: &SS::State,
                stop_vertex_state: &SS::State,
            ) -> (OrderedFloat<f32>, Self) {
                let me = Self {
                    dist_from_start: 0.0,
                };
                (
                    OrderedFloat(me.dist_from_start + my_vertex_state.dist(&stop_vertex_state)),
                    me,
                )
            }

            fn as_adj(
                prev_vertex_state: &SS::State,
                my_vertex_state: &SS::State,
                stop_vertex_state: &SS::State,
                parent: &Self,
            ) -> (OrderedFloat<f32>, Self) {
                let me = Self {
                    dist_from_start: parent.dist_from_start
                        + prev_vertex_state.dist(&my_vertex_state),
                };
                (
                    OrderedFloat(me.dist_from_start + my_vertex_state.dist(&stop_vertex_state)),
                    me,
                )
            }
        }

        #[derive(Debug)]
        pub struct W2AStarLike {
            dist_from_start: f32,
        }

        impl<SS: StateSpace> Propagation<SS> for W2AStarLike {
            fn as_start(
                my_vertex_state: &SS::State,
                stop_vertex_state: &SS::State,
            ) -> (OrderedFloat<f32>, Self) {
                let me = Self {
                    dist_from_start: 0.0,
                };
                (
                    OrderedFloat(me.dist_from_start + my_vertex_state.dist(&stop_vertex_state)),
                    me,
                )
            }

            fn as_adj(
                prev_vertex_state: &SS::State,
                my_vertex_state: &SS::State,
                stop_vertex_state: &SS::State,
                parent: &Self,
            ) -> (OrderedFloat<f32>, Self) {
                let me = Self {
                    dist_from_start: parent.dist_from_start
                        + prev_vertex_state.dist(&my_vertex_state),
                };
                (
                    OrderedFloat(
                        me.dist_from_start + my_vertex_state.dist(&stop_vertex_state) * 2.0,
                    ),
                    me,
                )
            }
        }
    }

    struct CostPriority {
        vertex_idx: usize,
        vertex_cost: OrderedFloat<f32>,
    }

    impl PartialEq for CostPriority {
        fn eq(&self, other: &Self) -> bool {
            self.vertex_cost == other.vertex_cost
        }
    }

    impl Eq for CostPriority {}

    impl Ord for CostPriority {
        fn cmp(&self, other: &Self) -> Ordering {
            self.vertex_cost.cmp(&other.vertex_cost)
        }
    }

    impl PartialOrd for CostPriority {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug)]
    pub struct TreeSearch<'a, SS: StateSpace> {
        pub(crate) graph: &'a Graph<SS>,
        start_idx: usize,
        stop_idx: usize,
        pub(crate) parent_map: HashMap<usize, Option<usize>>,
        pub(crate) fringe: HashSet<usize>,
        // tree: HashMap<usize, F>,
    }

    impl<'a, SS: StateSpace> TreeSearch<'a, SS> {
        pub fn try_search<F: Propagation<SS>>(
            graph: &'a Graph<SS>,
            start_idx: usize,
            stop_idx: usize,
        ) -> TreeSearch<'a, SS> {
            Self::try_search_with_alloc::<F>(graph, start_idx, stop_idx, 1.0)
        }

        pub fn try_search_with_alloc<F: Propagation<SS>>(
            graph: &'a Graph<SS>,
            start_idx: usize,
            stop_idx: usize,
            initial_alloc_frac: f32,
        ) -> Self {
            assert!(start_idx < graph.vertices.len());
            assert!(stop_idx < graph.vertices.len());
            assert!(initial_alloc_frac >= 0.0);
            let (start_cost, start_search_state) = F::as_start(
                &graph.vertices[start_idx].state,
                &graph.vertices[stop_idx].state,
            );
            let collec_alloc_size = (graph.vertices.len() as f32 * initial_alloc_frac) as usize;
            let mut parent_map = HashMap::with_capacity(collec_alloc_size);
            parent_map.insert(start_idx, None);
            let mut tree = HashMap::with_capacity(collec_alloc_size);
            tree.insert(start_idx, start_search_state);
            let mut fringe = BinaryHeap::with_capacity(collec_alloc_size);
            fringe.push(Reverse(CostPriority {
                vertex_idx: start_idx,
                vertex_cost: start_cost,
            }));
            while let Some(Reverse(CostPriority {
                vertex_idx: curr_idx,
                ..
            })) = fringe.pop()
            {
                if curr_idx == stop_idx {
                    break;
                }
                for &adj_idx in graph.vertices[curr_idx].adjacencies.iter() {
                    if let None = tree.get(&adj_idx) {
                        let (adj_cost, adj_search_state) = F::as_adj(
                            &graph.vertices[curr_idx].state,
                            &graph.vertices[adj_idx].state,
                            &graph.vertices[stop_idx].state,
                            &tree[&curr_idx],
                        );
                        parent_map.insert(adj_idx, Some(curr_idx));
                        tree.insert(adj_idx, adj_search_state);
                        fringe.push(Reverse(CostPriority {
                            vertex_idx: adj_idx,
                            vertex_cost: adj_cost,
                        }));
                    }
                }
            }
            TreeSearch {
                graph: graph,
                start_idx: start_idx,
                stop_idx: stop_idx,
                parent_map: parent_map,
                fringe: fringe
                    .into_sorted_vec()
                    .into_iter()
                    .map(|Reverse(CostPriority { vertex_idx, .. })| vertex_idx)
                    .collect(),
                // tree: tree,
            }
        }

        pub fn start_idx(&self) -> usize {
            self.start_idx
        }

        pub fn stop_idx(&self) -> usize {
            self.stop_idx
        }

        pub fn max_idx(&self) -> usize {
            self.graph.vertices.len() - 1
        }

        pub fn path_to_stop(&self) -> Option<Vec<usize>> {
            self.path_to(self.stop_idx)
        }

        pub fn path_to(&self, goal_idx: usize) -> Option<Vec<usize>> {
            assert!(goal_idx <= self.max_idx());
            if goal_idx == self.start_idx {
                return Some(vec![self.start_idx]);
            }
            let mut vertex_idx = goal_idx;
            let mut path = vec![vertex_idx];
            while let Some(&Some(parent_idx)) = self.parent_map.get(&vertex_idx) {
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
