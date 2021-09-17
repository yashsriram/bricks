use crate::spaces::*;
use vz::bevy::prelude::*;
use vz::bevy::render::mesh::{Indices, Mesh};
use vz::bevy::render::pipeline::{PrimitiveTopology, RenderPipelines};
use vz::plugins::{FILL_PIPELINE, NON_FILL_PIPELINE};
use vz::AsEntity;

impl AsEntity for RectangleSpace {
    fn into_mesh_bundles(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Vec<MeshBundle> {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        let positions = vec![
            [0.0, 0.0, 0.0],
            [self.size.x, 0.0, 0.0],
            [self.size.x, self.size.y, 0.0],
            [0.0, self.size.y, 0.0],
        ];
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        let indices = Indices::U32(vec![0, 1, 2, 3, 0]);
        mesh.set_indices(Some(indices));
        let colors = vec![[1.0, 1.0, 0.0, 0.1]; 4];
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        vec![MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
            draw: Default::default(),
            visible: Default::default(),
            main_pass: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }]
    }
}

impl AsEntity for CircleSpace {
    fn into_mesh_bundles(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Vec<MeshBundle> {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        let num_partitions: usize = 18;
        let positions: Vec<[f32; 3]> = (0..=num_partitions)
            .map(|i| 2.0 * std::f32::consts::PI / num_partitions as f32 * i as f32)
            .map(|theta| [self.radius * theta.cos(), self.radius * theta.sin(), 0.0])
            .collect();
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        let indices = Indices::U32((0..=num_partitions).map(|i| i as u32).collect());
        mesh.set_indices(Some(indices));
        let colors = vec![[1.0, 1.0, 0.0, 0.1]; num_partitions + 1];
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        vec![MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
            draw: Default::default(),
            visible: Default::default(),
            main_pass: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }]
    }
}

impl AsEntity for CuboidSpace {
    fn into_mesh_bundles(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Vec<MeshBundle> {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        let vertex_positions = vec![
            [0.0, 0.0, 0.0],
            [self.size.x, 0.0, 0.0],
            [self.size.x, self.size.y, 0.0],
            [0.0, self.size.y, 0.0],
            [0.0, 0.0, self.size.z],
            [self.size.x, 0.0, self.size.z],
            [self.size.x, self.size.y, self.size.z],
            [0.0, self.size.y, self.size.z],
        ];
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertex_positions);
        let indices = Indices::U32(vec![
            0, 1, 1, 2, 2, 3, 3, 0, 4, 5, 5, 6, 6, 7, 7, 4, 0, 4, 1, 5, 2, 6, 3, 7,
        ]);
        mesh.set_indices(Some(indices));
        let vertex_colors = vec![[1.0, 1.0, 0.0, 0.1]; 8];
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);
        vec![MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
            draw: Default::default(),
            visible: Default::default(),
            main_pass: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }]
    }
}

impl AsEntity for SphereSpace {
    fn into_mesh_bundles(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Vec<MeshBundle> {
        let mut mesh: Mesh = shape::Icosphere {
            radius: self.radius,
            subdivisions: 10,
        }
        .into();
        mesh.set_attribute(
            Mesh::ATTRIBUTE_COLOR,
            vec![[1.0, 1.0, 0.0, 0.1]; mesh.count_vertices()],
        );
        vec![MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
            draw: Default::default(),
            visible: Default::default(),
            main_pass: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }]
    }
}

use crate::graph::*;
use crate::{State, StateSpace};

impl<S: StateSpace> AsEntity for Graph<S> {
    fn into_mesh_bundles(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Vec<MeshBundle> {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        let positions: Vec<[f32; 3]> = self
            .vertices
            .iter()
            .map(|Vertex { state, .. }| state.project_to_3d())
            .collect();
        let indices: Vec<u32> = self
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
        let colors = vec![[1.0, 1.0, 1.0, 0.1]; self.vertices.len()];
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        vec![MeshBundle {
            mesh: meshes.add(mesh),
            render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
            draw: Default::default(),
            visible: Default::default(),
            main_pass: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }]
    }
}

use crate::graph::search::spanning::*;

impl<'a, SS: StateSpace> AsEntity for TreeSearch<'a, SS> {
    fn into_mesh_bundles(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Vec<MeshBundle> {
        let mut search_mesh = Mesh::new(PrimitiveTopology::LineList);
        let flattened_tree: Vec<usize> = self
            .parent_map
            .iter()
            .map(|(&child_idx, &parent_idx)| vec![child_idx, parent_idx.unwrap_or(child_idx)])
            .flatten()
            .collect();
        let positions: Vec<[f32; 3]> = flattened_tree
            .iter()
            .map(|idx| self.graph.vertices[*idx].state.project_to_3d())
            .collect();
        let box_size = positions
            .chunks_exact(2)
            .map(|chunk| (Vec3::from(chunk[0]) - Vec3::from(chunk[1])).length())
            .sum::<f32>()
            / (positions.len() as f32 / 2.0)
            / 2.0;
        let indices: Vec<u32> = positions
            .iter()
            .enumerate()
            .map(|(i, _)| i as u32)
            .collect();
        let indices = Indices::U32(indices);
        search_mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        search_mesh.set_indices(Some(indices));
        let vertex_colors: Vec<[f32; 4]> = flattened_tree
            .iter()
            .map(|idx| {
                if *idx == self.start_idx() {
                    [1.0, 1.0, 0.0, 1.0]
                } else if *idx == self.stop_idx() {
                    [0.0, 1.0, 0.0, 1.0]
                } else if self.fringe.contains(idx) {
                    [0.0, 1.0, 1.0, 1.0]
                } else {
                    [1.0, 0.0, 1.0, 0.2]
                }
            })
            .collect();
        search_mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);
        vec![
            MeshBundle {
                mesh: meshes.add(search_mesh),
                render_pipelines: RenderPipelines::from_handles(&[NON_FILL_PIPELINE.typed()]),
                draw: Default::default(),
                visible: Default::default(),
                main_pass: Default::default(),
                transform: Default::default(),
                global_transform: Default::default(),
            },
            MeshBundle {
                mesh: meshes.add({
                    let mut mesh: Mesh = shape::Box::new(box_size, box_size, box_size).into();
                    mesh.set_attribute(
                        Mesh::ATTRIBUTE_COLOR,
                        vec![[0.0, 1.0, 0.0, 1.0]; mesh.count_vertices()],
                    );
                    mesh
                }),
                render_pipelines: RenderPipelines::from_handles(&[FILL_PIPELINE.typed()]),
                draw: Default::default(),
                visible: Default::default(),
                main_pass: Default::default(),
                transform: Transform::from_translation(
                    self.graph.vertices[self.start_idx()]
                        .state
                        .project_to_3d()
                        .into(),
                ),
                global_transform: Default::default(),
            },
            MeshBundle {
                mesh: meshes.add({
                    let mut mesh: Mesh = shape::Box::new(box_size, box_size, box_size).into();
                    mesh.set_attribute(
                        Mesh::ATTRIBUTE_COLOR,
                        vec![[1.0, 0.0, 0.0, 1.0]; mesh.count_vertices()],
                    );
                    mesh
                }),
                render_pipelines: RenderPipelines::from_handles(&[FILL_PIPELINE.typed()]),
                draw: Default::default(),
                visible: Default::default(),
                main_pass: Default::default(),
                transform: Transform::from_translation(
                    self.graph.vertices[self.stop_idx()]
                        .state
                        .project_to_3d()
                        .into(),
                ),
                global_transform: Default::default(),
            },
        ]
    }
}
