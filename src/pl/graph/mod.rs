use super::StateSpace;
use std::collections::HashSet;

pub mod path;
pub mod prm;
pub mod search;

#[derive(Debug)]
pub struct Vertex<S: StateSpace> {
    pub(crate) state: S::State,
    pub(crate) adjacencies: HashSet<usize>,
}

#[derive(Debug)]
pub struct Graph<S: StateSpace> {
    pub(crate) vertices: Vec<Vertex<S>>,
}

impl<S: StateSpace> Graph<S> {
    pub fn num_edges(&self) -> usize {
        self.vertices
            .iter()
            .map(
                |Vertex {
                     state: _,
                     adjacencies,
                 }| adjacencies.len(),
            )
            .sum()
    }
}

use super::State;
use crate::vz::bevy::prelude::*;
use crate::vz::bevy::render::mesh::{Indices, Mesh};
use crate::vz::bevy::render::pipeline::{PrimitiveTopology, RenderPipelines};
use crate::vz::plugins::NON_FILL_PIPELINE;
use crate::vz::AsEntity;

impl<SS: StateSpace> AsEntity for Graph<SS> {
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
