use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, Mesh, VertexAttributeValues},
        pipeline::PrimitiveTopology,
    },
};

#[derive(Debug)]
pub struct DiffDrive {
    pub radius: f32,
}

impl DiffDrive {
    const POLYGON_SIZE: usize = 18;

    pub fn update(transform: &mut Transform, v: f32, w: f32, dt: f32) {
        let (axis, angle) = transform.rotation.to_axis_angle();
        let orient_in_rad = axis.z.signum() * angle;
        transform.translation.x += v * orient_in_rad.cos() * dt;
        transform.translation.y += v * orient_in_rad.sin() * dt;
        transform.rotation *= Quat::from_rotation_z(w * dt);
    }
}

impl From<&DiffDrive> for Mesh {
    fn from(diff_drive: &DiffDrive) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, {
            let mut positions: Vec<[f32; 3]> = (0..=DiffDrive::POLYGON_SIZE)
                .map(|i| 2.0 * std::f32::consts::PI / DiffDrive::POLYGON_SIZE as f32 * i as f32)
                .map(|theta| {
                    [
                        diff_drive.radius * theta.cos(),
                        diff_drive.radius * theta.sin(),
                        0.0,
                    ]
                })
                .collect();
            positions.push([0.0, 0.0, 0.0]);
            positions
        });
        mesh.set_attribute(
            Mesh::ATTRIBUTE_COLOR,
            vec![[1.0, 1.0, 1.0, 1.0]; DiffDrive::POLYGON_SIZE + 2],
        );
        mesh.set_indices(Some(Indices::U32(
            (0..=(DiffDrive::POLYGON_SIZE + 1))
                .map(|i| i as u32)
                .collect(),
        )));
        mesh
    }
}

pub struct Path {
    pub len: usize,
}

impl Path {
    pub fn add_point(&self, mesh: &mut Mesh, point: [f32; 3]) {
        if let Some(VertexAttributeValues::Float3(ref mut vec)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            vec.push(point);
        }
        if let Some(VertexAttributeValues::Float4(ref mut vec)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR)
        {
            vec.push([1.0, 1.0, 1.0, 0.2]);
        }
        if let Some(Indices::U32(ref mut vec)) = mesh.indices_mut() {
            vec.push(vec.len() as u32);
        }
    }
}

impl From<&Path> for Mesh {
    fn from(_: &Path) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0.0, 0.0, 0.0]]);
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, vec![[0.0, 0.0, 1.0, 0.1]]);
        mesh.set_indices(Some(Indices::U32(vec![0])));
        mesh
    }
}
