pub mod graph;
pub mod path;
pub mod prm;
pub mod search;

use bevy::render::{
    mesh::{Indices, Mesh},
    pipeline::PrimitiveTopology,
};
use nalgebra::Vector3;

#[derive(Debug)]
pub struct CuboidSpace {
    pub size: Vector3<f32>,
}

impl From<&CuboidSpace> for Mesh {
    fn from(space: &CuboidSpace) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        let vertex_positions = vec![
            [0.0, 0.0, 0.0],
            [space.size.x, 0.0, 0.0],
            [space.size.x, space.size.y, 0.0],
            [0.0, space.size.y, 0.0],
            [0.0, 0.0, space.size.z],
            [space.size.x, 0.0, space.size.z],
            [space.size.x, space.size.y, space.size.z],
            [0.0, space.size.y, space.size.z],
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
