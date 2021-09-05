pub mod tree {
    use super::super::Graph;
    use crate::plan::{State, StateSpace};
    use bevy::render::mesh::{Indices, Mesh};
    use bevy::render::pipeline::PrimitiveTopology;
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::fmt::Debug;

    pub trait Propagation<S: StateSpace>: Debug + Sized {
        fn as_start(my_vertex_idx: usize, my_vertex_state: &S::State) -> (f32, Self);

        fn as_adj(
            prev_vertex_idx: usize,
            prev_vertex_state: &S::State,
            my_vertex_idx: usize,
            my_vertex_state: &S::State,
            parent: &Self,
        ) -> (f32, Self);
    }

    pub mod propagations {
        use super::Propagation;
        use crate::plan::{State, StateSpace};

        #[derive(Debug)]
        pub struct JumpsFromStart {
            jumps: usize,
        }

        impl<S: StateSpace> Propagation<S> for JumpsFromStart {
            fn as_start(_: usize, _: &S::State) -> (f32, Self) {
                let me = Self { jumps: 0 };
                (me.jumps as f32, me)
            }

            fn as_adj(
                _: usize,
                _: &S::State,
                _: usize,
                _: &S::State,
                parent: &Self,
            ) -> (f32, Self) {
                let me = Self {
                    jumps: parent.jumps + 1,
                };
                (me.jumps as f32, me)
            }
        }

        #[derive(Debug)]
        pub struct DistFromStart {
            dist: f32,
        }

        impl<S: StateSpace> Propagation<S> for DistFromStart {
            fn as_start(_start_vertex_idx: usize, _start_vertex_state: &S::State) -> (f32, Self) {
                let me = Self { dist: 0.0 };
                (me.dist as f32, me)
            }

            fn as_adj(
                _prev_vertex_idx: usize,
                prev_vertex_state: &S::State,
                _my_vertex_idx: usize,
                my_vertex_state: &S::State,
                parent: &Self,
            ) -> (f32, Self) {
                let me = Self {
                    dist: parent.dist + prev_vertex_state.dist(&my_vertex_state),
                };
                (me.dist as f32, me)
            }
        }
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
    pub struct TreeSearch<'a, S: StateSpace, F: Propagation<S>> {
        graph: &'a Graph<S>,
        start_idx: usize,
        stop_idx: usize,
        parent_map: HashMap<usize, Option<usize>>,
        fringe: HashSet<usize>,
        tree: HashMap<usize, F>,
    }

    impl<'a, S: StateSpace, F: Propagation<S>> TreeSearch<'a, S, F> {
        pub fn try_search(graph: &'a Graph<S>, start_idx: usize, stop_idx: usize) -> Self {
            Self::try_search_with_alloc(graph, start_idx, stop_idx, 1.0)
        }

        pub fn try_search_with_alloc(
            graph: &'a Graph<S>,
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
            let mut parent_map = HashMap::with_capacity(collec_alloc_size);
            parent_map.insert(start_idx, None);
            let mut tree = HashMap::with_capacity(collec_alloc_size);
            tree.insert(start_idx, start_search_state);
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
                    if let None = tree.get(&adj_idx) {
                        let (adj_cost, adj_search_state) = F::as_adj(
                            curr_idx,
                            &graph.vertices[curr_idx].state,
                            adj_idx,
                            &graph.vertices[adj_idx].state,
                            &tree[&curr_idx],
                        );
                        parent_map.insert(adj_idx, Some(curr_idx));
                        tree.insert(adj_idx, adj_search_state);
                        fringe.push(CostPriority {
                            vertex_idx: adj_idx,
                            vertex_cost: adj_cost,
                        });
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
                    .map(|CostPriority { vertex_idx, .. }| vertex_idx)
                    .collect(),
                tree: tree,
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

    impl<'a, S: StateSpace, F: Propagation<S>> From<TreeSearch<'a, S, F>> for Mesh {
        fn from(search: TreeSearch<'a, S, F>) -> Mesh {
            let mut mesh = Mesh::new(PrimitiveTopology::LineList);
            let flattened_tree: Vec<usize> = search
                .parent_map
                .iter()
                .map(|(&child_idx, &parent_idx)| vec![child_idx, parent_idx.unwrap_or(child_idx)])
                .flatten()
                .collect();
            let positions: Vec<[f32; 3]> = flattened_tree
                .iter()
                .map(|idx| search.graph.vertices[*idx].state.project_to_3d())
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
                    if *idx == search.start_idx {
                        [1.0, 1.0, 0.0, 1.0]
                    } else if *idx == search.stop_idx {
                        [0.0, 1.0, 0.0, 1.0]
                    } else if search.fringe.contains(idx) {
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
}
