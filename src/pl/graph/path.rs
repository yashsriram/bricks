use super::search::SpanningTreeView;
use super::{State, StateSpace};
use crate::vz::bevy::prelude::*;
use crate::vz::bevy::render::mesh::{Indices, Mesh};
use crate::vz::bevy::render::pipeline::{PrimitiveTopology, RenderPipelines};
use crate::vz::plugins::NON_FILL_PIPELINE;
use crate::vz::AsEntity;

#[derive(Debug)]
pub struct Path<SS: StateSpace> {
    pub(crate) vertices: Vec<SS::State>,
}

impl<S: StateSpace> Path<S> {
    pub fn len(&self) -> usize {
        self.vertices.len()
    }
}

impl<'a, SS: StateSpace> From<&SpanningTreeView<'a, SS>> for Path<SS> {
    fn from(ts: &SpanningTreeView<'a, SS>) -> Path<SS> {
        let vertices = match ts.path_to_stop() {
            None => vec![],
            Some(path) => path
                .into_iter()
                .map(|idx| ts.graph.vertices[idx].state)
                .collect(),
        };
        Path { vertices }
    }
}

impl<SS: StateSpace> AsEntity for Path<SS> {
    fn into_mesh_bundles(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Vec<MeshBundle> {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        let positions: Vec<[f32; 3]> = self.vertices.iter().map(|v| v.project_to_3d()).collect();
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        let indices = Indices::U32((0..self.vertices.len() as u32).collect());
        mesh.set_indices(Some(indices));
        let colors = vec![[0.0, 1.0, 0.0, 1.0]; self.vertices.len()];
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
