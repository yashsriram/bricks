use super::search::CostGuidedTreeSearchResult;
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::pipeline::PrimitiveTopology;
use nalgebra::Point3;

#[derive(Debug)]
pub struct Path {
    pub(crate) vertices: Vec<Point3<f32>>,
}

impl Path {
    pub fn len(&self) -> usize {
        self.vertices.len()
    }
}

impl<'a> From<&CostGuidedTreeSearchResult<'a>> for Path {
    fn from(ts: &CostGuidedTreeSearchResult<'a>) -> Path {
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

impl From<&Path> for Mesh {
    fn from(path: &Path) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        let positions: Vec<[f32; 3]> = path.vertices.iter().map(|v| [v.x, v.y, v.z]).collect();
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        let indices = Indices::U32((0..path.vertices.len() as u32).collect());
        mesh.set_indices(Some(indices));
        let colors = vec![[0.0, 1.0, 0.0, 1.0]; path.vertices.len()];
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh
    }
}
