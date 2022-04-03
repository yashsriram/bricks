use super::Graph;
use super::super::StateSpace;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;

struct CostOrdAndIndex<Cost: Ord> {
    idx: usize,
    cost: Cost,
}

impl<Cost: Ord> PartialEq for CostOrdAndIndex<Cost> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<Cost: Ord> Eq for CostOrdAndIndex<Cost> {}

impl<Cost: Ord> Ord for CostOrdAndIndex<Cost> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl<Cost: Ord> PartialOrd for CostOrdAndIndex<Cost> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct SpanningTreeView<'a, SS: StateSpace> {
    pub(crate) graph: &'a Graph<SS>,
    start_idx: usize,
    stop_idx: usize,
    pub(crate) parent_map: HashMap<usize, Option<usize>>,
    pub(crate) fringe: HashSet<usize>,
    // tree: HashMap<usize, SC>,
}

impl<'a, SS: StateSpace> SpanningTreeView<'a, SS> {
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
        let mut idx = goal_idx;
        let mut path = vec![idx];
        while let Some(&Some(parent_idx)) = self.parent_map.get(&idx) {
            path.push(parent_idx);
            idx = parent_idx;
        }
        let path: Vec<usize> = path.into_iter().rev().collect();
        match path.len() {
            1 => None,
            _ => Some(path),
        }
    }
}

pub trait CostGuidedSpanningTreeSearch<SS: StateSpace, Cost: Ord>: Debug + Sized {
    fn as_start(my_vertex_state: &SS::State, stop_vertex_state: &SS::State) -> Self;

    fn as_adj(
        prev_vertex_state: &SS::State,
        my_vertex_state: &SS::State,
        stop_vertex_state: &SS::State,
        parent: &Self,
    ) -> Self;

    fn cost(&self) -> Cost;

    fn try_on<'a>(
        graph: &'a Graph<SS>,
        start_idx: usize,
        stop_idx: usize,
    ) -> SpanningTreeView<'a, SS> {
        Self::try_on_with_alloc(graph, start_idx, stop_idx, 1.0)
    }

    fn try_on_with_alloc<'a>(
        graph: &'a Graph<SS>,
        start_idx: usize,
        stop_idx: usize,
        initial_alloc_frac: f32,
    ) -> SpanningTreeView<'a, SS> {
        assert!(start_idx < graph.vertices.len());
        assert!(stop_idx < graph.vertices.len());
        assert!(initial_alloc_frac >= 0.0);
        let start_search_state = Self::as_start(
            &graph.vertices[start_idx].state,
            &graph.vertices[stop_idx].state,
        );
        let collec_alloc_size = (graph.vertices.len() as f32 * initial_alloc_frac) as usize;
        let mut parent_map = HashMap::with_capacity(collec_alloc_size);
        parent_map.insert(start_idx, None);
        let mut fringe = BinaryHeap::with_capacity(collec_alloc_size);
        fringe.push(Reverse(CostOrdAndIndex {
            idx: start_idx,
            cost: start_search_state.cost(),
        }));
        let mut tree = HashMap::with_capacity(collec_alloc_size);
        tree.insert(start_idx, start_search_state);
        while let Some(Reverse(CostOrdAndIndex { idx: curr_idx, .. })) = fringe.pop() {
            if curr_idx == stop_idx {
                break;
            }
            for &adj_idx in graph.vertices[curr_idx].adjacencies.iter() {
                if let None = tree.get(&adj_idx) {
                    let adj_search_state = Self::as_adj(
                        &graph.vertices[curr_idx].state,
                        &graph.vertices[adj_idx].state,
                        &graph.vertices[stop_idx].state,
                        &tree[&curr_idx],
                    );
                    parent_map.insert(adj_idx, Some(curr_idx));
                    fringe.push(Reverse(CostOrdAndIndex {
                        idx: adj_idx,
                        cost: adj_search_state.cost(),
                    }));
                    tree.insert(adj_idx, adj_search_state);
                }
            }
        }
        SpanningTreeView {
            graph,
            start_idx,
            stop_idx,
            parent_map,
            fringe: fringe
                .into_sorted_vec()
                .into_iter()
                .map(|Reverse(CostOrdAndIndex { idx, .. })| idx)
                .collect(),
            // tree: tree,
        }
    }
}

pub mod spanning_trees {
    use super::CostGuidedSpanningTreeSearch;
    use super::super::super::{State, StateSpace};
    use ordered_float::OrderedFloat;

    #[derive(Debug)]
    pub struct DFSLike {
        order: isize,
    }

    impl<SS: StateSpace> CostGuidedSpanningTreeSearch<SS, isize> for DFSLike {
        fn as_start(_: &SS::State, _: &SS::State) -> Self {
            Self { order: -0 }
        }

        fn as_adj(_: &SS::State, _: &SS::State, _: &SS::State, parent: &Self) -> Self {
            Self {
                order: parent.order - 1,
            }
        }

        fn cost(&self) -> isize {
            self.order
        }
    }

    #[derive(Debug)]
    pub struct BFSLike {
        jumps_from_start: usize,
    }

    impl<SS: StateSpace> CostGuidedSpanningTreeSearch<SS, usize> for BFSLike {
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

        fn cost(&self) -> usize {
            self.jumps_from_start
        }
    }

    #[derive(Debug)]
    pub struct WeightedAStarLike<const NUM: usize, const DEN: usize> {
        dist_from_start: f32,
        total_cost: f32,
    }

    impl<SS: StateSpace, const NUM: usize, const DEN: usize>
        CostGuidedSpanningTreeSearch<SS, OrderedFloat<f32>> for WeightedAStarLike<NUM, DEN>
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
            let dist_from_start = parent.dist_from_start + prev_vertex_state.dist(&my_vertex_state);
            Self {
                dist_from_start,
                total_cost: dist_from_start
                    + my_vertex_state.dist(&stop_vertex_state) * (NUM as f32 / DEN as f32),
            }
        }

        fn cost(&self) -> OrderedFloat<f32> {
            OrderedFloat(self.total_cost)
        }
    }

    pub type UCSLike = WeightedAStarLike<0, 1>;
    pub type AStarLike = WeightedAStarLike<1, 1>;
    pub type W2AStarLike = WeightedAStarLike<2, 1>;
}
