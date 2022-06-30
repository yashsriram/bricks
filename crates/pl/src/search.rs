use crate::graph::Graph;
use bevy::render::{
    mesh::{Indices, Mesh},
    pipeline::PrimitiveTopology,
};
use nalgebra::Point3;
use ordered_float::OrderedFloat;
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
};

#[derive(Debug)]
pub struct CostGuidedTreeSearchResult<'a> {
    pub(crate) graph: &'a Graph,
    start_idx: usize,
    stop_idx: usize,
    reached: bool,
    pub(crate) parent_map: HashMap<usize, Option<usize>>,
    pub(crate) fringe: HashSet<usize>,
}

impl<'a> CostGuidedTreeSearchResult<'a> {
    pub fn sucess(&self) -> bool {
        self.reached
    }

    pub fn path_to_stop(&self) -> Option<Vec<usize>> {
        self.path_to(self.stop_idx)
    }

    pub fn path_to(&self, goal_idx: usize) -> Option<Vec<usize>> {
        assert!(goal_idx <= self.graph.vertices.len() - 1);
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

impl<'a> From<&CostGuidedTreeSearchResult<'a>> for Mesh {
    fn from(spanning_tree_view: &CostGuidedTreeSearchResult<'a>) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        let flattened_tree: Vec<usize> = spanning_tree_view
            .parent_map
            .iter()
            .map(|(&child_idx, &parent_idx)| vec![child_idx, parent_idx.unwrap_or(child_idx)])
            .flatten()
            .collect();
        let positions: Vec<[f32; 3]> = flattened_tree
            .iter()
            .map(|idx| {
                let state = spanning_tree_view.graph.vertices[*idx].state;
                [state.x, state.y, state.z]
            })
            .collect();
        let indices: Vec<u32> = positions
            .iter()
            .enumerate()
            .map(|(i, _)| i as u32)
            .collect();
        let indices = Indices::U32(indices);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_indices(Some(indices));
        let vertex_colors: Vec<[f32; 4]> = flattened_tree
            .iter()
            .map(|idx| {
                if *idx == spanning_tree_view.start_idx {
                    [1.0, 1.0, 0.0, 1.0]
                } else if *idx == spanning_tree_view.stop_idx {
                    [0.0, 1.0, 0.0, 1.0]
                } else if spanning_tree_view.fringe.contains(idx) {
                    [0.0, 1.0, 1.0, 1.0]
                } else {
                    [1.0, 0.0, 1.0, 0.2]
                }
            })
            .collect();
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);
        mesh
    }
}

pub trait CostGuidedWaveTreeSearch<Cost: Ord>: Debug + Sized {
    fn as_start(my_vertex_state: &Point3<f32>, stop_vertex_state: &Point3<f32>) -> Self;

    fn as_adj(
        prev_vertex_state: &Point3<f32>,
        my_vertex_state: &Point3<f32>,
        stop_vertex_state: &Point3<f32>,
        parent: &Self,
    ) -> Self;

    fn cost(&self) -> Cost;

    fn try_on<'a>(
        graph: &'a Graph,
        start_idx: usize,
        stop_idx: usize,
    ) -> CostGuidedTreeSearchResult<'a> {
        Self::try_on_with_alloc(graph, start_idx, stop_idx, 1.0)
    }

    fn try_on_with_alloc<'a>(
        graph: &'a Graph,
        start_idx: usize,
        stop_idx: usize,
        initial_alloc_frac: f32,
    ) -> CostGuidedTreeSearchResult<'a> {
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

        fringe.push(Reverse(CostOrdAndIndex {
            idx: start_idx,
            cost: start_search_state.cost(),
        }));
        let mut tree = HashMap::with_capacity(collec_alloc_size);
        tree.insert(start_idx, start_search_state);
        while let Some(Reverse(CostOrdAndIndex { idx: curr_idx, .. })) = fringe.pop() {
            if curr_idx == stop_idx {
                return CostGuidedTreeSearchResult {
                    graph,
                    start_idx,
                    stop_idx,
                    parent_map,
                    fringe: fringe
                        .into_sorted_vec()
                        .into_iter()
                        .map(|Reverse(CostOrdAndIndex { idx, .. })| idx)
                        .collect(),
                    reached: true,
                };
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
        CostGuidedTreeSearchResult {
            graph,
            start_idx,
            stop_idx,
            parent_map,
            fringe: fringe
                .into_sorted_vec()
                .into_iter()
                .map(|Reverse(CostOrdAndIndex { idx, .. })| idx)
                .collect(),
            reached: false,
        }
    }
}

#[derive(Debug)]
pub struct DFS {
    order: isize,
}

impl CostGuidedWaveTreeSearch<isize> for DFS {
    fn as_start(_: &Point3<f32>, _: &Point3<f32>) -> Self {
        Self { order: -0 }
    }

    fn as_adj(_: &Point3<f32>, _: &Point3<f32>, _: &Point3<f32>, parent: &Self) -> Self {
        Self {
            order: parent.order - 1,
        }
    }

    fn cost(&self) -> isize {
        self.order
    }
}

#[derive(Debug)]
pub struct BFS {
    jumps_from_start: usize,
}

impl CostGuidedWaveTreeSearch<usize> for BFS {
    fn as_start(_: &Point3<f32>, _: &Point3<f32>) -> Self {
        Self {
            jumps_from_start: 0,
        }
    }

    fn as_adj(_: &Point3<f32>, _: &Point3<f32>, _: &Point3<f32>, parent: &Self) -> Self {
        Self {
            jumps_from_start: parent.jumps_from_start + 1,
        }
    }

    fn cost(&self) -> usize {
        self.jumps_from_start
    }
}

#[derive(Debug)]
pub struct WeightableAStar<const NUM: usize, const DEN: usize> {
    dist_from_start: f32,
    total_cost: f32,
}

impl<const NUM: usize, const DEN: usize> CostGuidedWaveTreeSearch<OrderedFloat<f32>>
    for WeightableAStar<NUM, DEN>
{
    fn as_start(my_vertex_state: &Point3<f32>, stop_vertex_state: &Point3<f32>) -> Self {
        Self {
            dist_from_start: 0.0,
            total_cost: 0.0 + (my_vertex_state - stop_vertex_state).norm(),
        }
    }

    fn as_adj(
        prev_vertex_state: &Point3<f32>,
        my_vertex_state: &Point3<f32>,
        stop_vertex_state: &Point3<f32>,
        parent: &Self,
    ) -> Self {
        let dist_from_start = parent.dist_from_start + (prev_vertex_state - my_vertex_state).norm();
        Self {
            dist_from_start,
            total_cost: dist_from_start
                + (my_vertex_state - stop_vertex_state).norm() * (NUM as f32 / DEN as f32),
        }
    }

    fn cost(&self) -> OrderedFloat<f32> {
        OrderedFloat(self.total_cost)
    }
}

pub type UCS = WeightableAStar<0, 1>;
pub type AStar = WeightableAStar<1, 1>;
pub type AStarWeighted2 = WeightableAStar<2, 1>;
