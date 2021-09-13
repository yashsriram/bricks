mod plugin;

pub use plugin::*;

use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::pipeline::PrimitiveTopology;

pub trait AsEntity: Sized
where
    Mesh: From<Self>,
{
    fn spawn(self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>) {
        commands.spawn_bundle(MeshBundle {
            mesh: meshes.add(self.into()),
            render_pipelines: RenderPipelines::from_handles(&[plugin::NON_FILL_PIPELINE.typed()]),
            ..Default::default()
        });
    }
}

use crate::plan::spaces::*;

impl From<RectangleSpace> for Mesh {
    fn from(s: RectangleSpace) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        let positions = vec![
            [0.0, 0.0, 0.0],
            [s.size.x, 0.0, 0.0],
            [s.size.x, s.size.y, 0.0],
            [0.0, s.size.y, 0.0],
        ];
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        let indices = Indices::U32(vec![0, 1, 2, 3, 0]);
        mesh.set_indices(Some(indices));
        let colors = vec![[1.0, 1.0, 0.0, 0.1]; 4];
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh
    }
}

impl AsEntity for RectangleSpace {}

impl From<CircleSpace> for Mesh {
    fn from(s: CircleSpace) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        let num_partitions: usize = 18;
        let positions: Vec<[f32; 3]> = (0..=num_partitions)
            .map(|i| 2.0 * std::f32::consts::PI / num_partitions as f32 * i as f32)
            .map(|theta| [s.radius * theta.cos(), s.radius * theta.sin(), 0.0])
            .collect();
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        let indices = Indices::U32((0..=num_partitions).map(|i| i as u32).collect());
        mesh.set_indices(Some(indices));
        let colors = vec![[1.0, 1.0, 0.0, 0.1]; num_partitions + 1];
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh
    }
}

impl AsEntity for CuboidSpace {}

impl From<CuboidSpace> for Mesh {
    fn from(s: CuboidSpace) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        let vertex_positions = vec![
            [0.0, 0.0, 0.0],
            [s.size.x, 0.0, 0.0],
            [s.size.x, s.size.y, 0.0],
            [0.0, s.size.y, 0.0],
            [0.0, 0.0, s.size.z],
            [s.size.x, 0.0, s.size.z],
            [s.size.x, s.size.y, s.size.z],
            [0.0, s.size.y, s.size.z],
        ];
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertex_positions);
        let indices = Indices::U32(vec![
            0, 1, 1, 2, 2, 3, 3, 0, 4, 5, 5, 6, 6, 7, 7, 4, 0, 4, 1, 5, 2, 6, 3, 7,
        ]);
        mesh.set_indices(Some(indices));
        let vertex_colors = vec![[1.0, 1.0, 0.0, 0.1]; 8];
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);
        mesh
    }
}

impl AsEntity for CircleSpace {}

impl From<SphereSpace> for Mesh {
    fn from(s: SphereSpace) -> Mesh {
        let mut mesh: Mesh = shape::Icosphere {
            radius: s.radius,
            subdivisions: 10,
        }
        .into();
        mesh.set_attribute(
            Mesh::ATTRIBUTE_COLOR,
            vec![[1.0, 1.0, 0.0, 0.1]; mesh.count_vertices()],
        );
        mesh
    }
}

impl AsEntity for SphereSpace {}

use crate::plan::graph::*;
use crate::plan::{State, StateSpace};

impl<S: StateSpace> From<Graph<S>> for Mesh {
    fn from(graph: Graph<S>) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        let positions: Vec<[f32; 3]> = graph
            .vertices
            .iter()
            .map(|Vertex { state, .. }| state.project_to_3d())
            .collect();
        let indices: Vec<u32> = graph
            .vertices
            .iter()
            .enumerate()
            .map(
                |(
                    v_i,
                    Vertex {
                        state: _,
                        adjacencies: v_adjs,
                    },
                )| {
                    v_adjs
                        .iter()
                        .map(move |&adj| vec![v_i as u32, adj as u32])
                        .flatten()
                },
            )
            .into_iter()
            .flatten()
            .collect();
        let indices = Indices::U32(indices);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_indices(Some(indices));
        let colors = vec![[1.0, 1.0, 1.0, 0.1]; graph.vertices.len()];
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh
    }
}

use crate::plan::graph::search::spanning::*;

impl<'a, SS: StateSpace, F: Propagation<SS>> From<TreeSearch<'a, SS, F>> for Mesh {
    fn from(search: TreeSearch<'a, SS, F>) -> Mesh {
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
                if *idx == search.start_idx() {
                    [1.0, 1.0, 0.0, 1.0]
                } else if *idx == search.stop_idx() {
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
