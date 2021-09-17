pub mod spanning {
    use super::super::Graph;
    use crate::StateSpace;
    use std::cmp::Ordering;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::fmt::Debug;

    pub trait StateCost<SS: StateSpace, Cost: Ord>: Debug + Sized {
        fn as_start(my_vertex_state: &SS::State, stop_vertex_state: &SS::State) -> Self;

        fn as_adj(
            prev_vertex_state: &SS::State,
            my_vertex_state: &SS::State,
            stop_vertex_state: &SS::State,
            parent: &Self,
        ) -> Self;

        fn cost(&self) -> Cost;
    }

    pub mod propagations {
        use super::StateCost;
        use crate::{State, StateSpace};
        use ordered_float::OrderedFloat;

        #[derive(Debug)]
        pub struct DFSLike {
            order: isize,
        }

        impl<SS: StateSpace> StateCost<SS, OrderedFloat<f32>> for DFSLike {
            fn as_start(_: &SS::State, _: &SS::State) -> Self {
                Self { order: -0 }
            }

            fn as_adj(_: &SS::State, _: &SS::State, _: &SS::State, parent: &Self) -> Self {
                Self {
                    order: parent.order - 1,
                }
            }

            fn cost(&self) -> OrderedFloat<f32> {
                OrderedFloat(self.order as f32)
            }
        }

        #[derive(Debug)]
        pub struct BFSLike {
            jumps_from_start: usize,
        }

        impl<SS: StateSpace> StateCost<SS, OrderedFloat<f32>> for BFSLike {
            fn as_start(_: &SS::State, _: &SS::State) -> Self {
                Self {
                    jumps_from_start: 0,
                }
            }

            fn as_adj(_: &SS::State, _: &SS::State, _: &SS::State, parent: &Self) -> Self {
                Self {
                    jumps_from_start: parent.jumps_from_start + 1,
                }
            }

            fn cost(&self) -> OrderedFloat<f32> {
                OrderedFloat(self.jumps_from_start as f32)
            }
        }

        #[derive(Debug)]
        pub struct UCSLike {
            dist_from_start: f32,
        }

        impl<SS: StateSpace> StateCost<SS, OrderedFloat<f32>> for UCSLike {
            fn as_start(_: &SS::State, _: &SS::State) -> Self {
                Self {
                    dist_from_start: 0.0,
                }
            }

            fn as_adj(
                prev_vertex_state: &SS::State,
                my_vertex_state: &SS::State,
                _: &SS::State,
                parent: &Self,
            ) -> Self {
                Self {
                    dist_from_start: parent.dist_from_start
                        + prev_vertex_state.dist(&my_vertex_state),
                }
            }

            fn cost(&self) -> OrderedFloat<f32> {
                OrderedFloat(self.dist_from_start as f32)
            }
        }

        #[derive(Debug)]
        pub struct AStarLike {
            dist_from_start: f32,
            total_cost: f32,
        }

        impl<SS: StateSpace> StateCost<SS, OrderedFloat<f32>> for AStarLike {
            fn as_start(my_vertex_state: &SS::State, stop_vertex_state: &SS::State) -> Self {
                Self {
                    dist_from_start: 0.0,
                    total_cost: 0.0 + my_vertex_state.dist(&stop_vertex_state),
                }
            }

            fn as_adj(
                prev_vertex_state: &SS::State,
                my_vertex_state: &SS::State,
                stop_vertex_state: &SS::State,
                parent: &Self,
            ) -> Self {
                let dist_from_start =
                    parent.dist_from_start + prev_vertex_state.dist(&my_vertex_state);
                Self {
                    dist_from_start,
                    total_cost: dist_from_start + my_vertex_state.dist(&stop_vertex_state),
                }
            }

            fn cost(&self) -> OrderedFloat<f32> {
                OrderedFloat(self.total_cost as f32)
            }
        }

        #[derive(Debug)]
        pub struct W2AStarLike<const NUM: usize, const DEN: usize> {
            dist_from_start: f32,
            total_cost: f32,
        }

        impl<SS: StateSpace, const NUM: usize, const DEN: usize> StateCost<SS, OrderedFloat<f32>>
            for W2AStarLike<NUM, DEN>
        {
            fn as_start(my_vertex_state: &SS::State, stop_vertex_state: &SS::State) -> Self {
                Self {
                    dist_from_start: 0.0,
                    total_cost: 0.0 + my_vertex_state.dist(&stop_vertex_state),
                }
            }

            fn as_adj(
                prev_vertex_state: &SS::State,
                my_vertex_state: &SS::State,
                stop_vertex_state: &SS::State,
                parent: &Self,
            ) -> Self {
                let dist_from_start =
                    parent.dist_from_start + prev_vertex_state.dist(&my_vertex_state);
                Self {
                    dist_from_start,
                    total_cost: dist_from_start
                        + my_vertex_state.dist(&stop_vertex_state) * (NUM as f32 / DEN as f32),
                }
            }

            fn cost(&self) -> OrderedFloat<f32> {
                OrderedFloat(self.total_cost as f32)
            }
        }
    }

    struct CostPriority<Cost: Ord> {
        vertex_idx: usize,
        vertex_cost: Cost,
    }

    impl<Cost: Ord> PartialEq for CostPriority<Cost> {
        fn eq(&self, other: &Self) -> bool {
            self.vertex_cost == other.vertex_cost
        }
    }

    impl<Cost: Ord> Eq for CostPriority<Cost> {}

    impl<Cost: Ord> Ord for CostPriority<Cost> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.vertex_cost.cmp(&other.vertex_cost)
        }
    }

    impl<Cost: Ord> PartialOrd for CostPriority<Cost> {
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
        // tree: HashMap<usize, SC>,
    }

    impl<'a, SS: StateSpace> TreeSearch<'a, SS> {
        pub fn try_with<Cost: Ord, SC: StateCost<SS, Cost>>(
            graph: &'a Graph<SS>,
            start_idx: usize,
            stop_idx: usize,
        ) -> TreeSearch<'a, SS> {
            Self::try_with_alloc::<Cost, SC>(graph, start_idx, stop_idx, 1.0)
        }

        pub fn try_with_alloc<Cost: Ord, SC: StateCost<SS, Cost>>(
            graph: &'a Graph<SS>,
            start_idx: usize,
            stop_idx: usize,
            initial_alloc_frac: f32,
        ) -> Self {
            assert!(start_idx < graph.vertices.len());
            assert!(stop_idx < graph.vertices.len());
            assert!(initial_alloc_frac >= 0.0);
            let start_search_state = SC::as_start(
                &graph.vertices[start_idx].state,
                &graph.vertices[stop_idx].state,
            );
            let collec_alloc_size = (graph.vertices.len() as f32 * initial_alloc_frac) as usize;
            let mut parent_map = HashMap::with_capacity(collec_alloc_size);
            parent_map.insert(start_idx, None);
            let mut fringe = BinaryHeap::with_capacity(collec_alloc_size);
            fringe.push(Reverse(CostPriority {
                vertex_idx: start_idx,
                vertex_cost: start_search_state.cost(),
            }));
            let mut tree = HashMap::with_capacity(collec_alloc_size);
            tree.insert(start_idx, start_search_state);
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
                        let adj_search_state = SC::as_adj(
                            &graph.vertices[curr_idx].state,
                            &graph.vertices[adj_idx].state,
                            &graph.vertices[stop_idx].state,
                            &tree[&curr_idx],
                        );
                        parent_map.insert(adj_idx, Some(curr_idx));
                        fringe.push(Reverse(CostPriority {
                            vertex_idx: adj_idx,
                            vertex_cost: adj_search_state.cost(),
                        }));
                        tree.insert(adj_idx, adj_search_state);
                    }
                }
            }
            TreeSearch {
                graph,
                start_idx,
                stop_idx,
                parent_map,
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
