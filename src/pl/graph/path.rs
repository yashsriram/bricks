use super::search::SpanningTreeView;
use super::{State, StateSpace};
use crate::vz::bevy::render::mesh::{Indices, Mesh};
use crate::vz::bevy::render::pipeline::PrimitiveTopology;

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

impl<SS: StateSpace> From<&Path<SS>> for Mesh {
    fn from(path: &Path<SS>) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        let positions: Vec<[f32; 3]> = path.vertices.iter().map(|v| v.project_to_3d()).collect();
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        let indices = Indices::U32((0..path.vertices.len() as u32).collect());
        mesh.set_indices(Some(indices));
        let colors = vec![[0.0, 1.0, 0.0, 1.0]; path.vertices.len()];
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh
    }
}
