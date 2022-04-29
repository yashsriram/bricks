use crate::pl::{State, StateSpace};
use bevy::render::{
    mesh::{Indices, Mesh},
    pipeline::PrimitiveTopology,
};
use nalgebra::{Point3, Vector3};
use rand::{distributions::Standard, thread_rng, Rng};

impl State for Point3<f32> {
    fn dist(&self, other: &Self) -> f32 {
        (self - other).norm()
    }

    fn project_to_3d(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

#[derive(Debug)]
pub struct CuboidSpace {
    pub size: Vector3<f32>,
}

impl StateSpace for CuboidSpace {
    type State = Point3<f32>;

    fn sample_batch(&self, num_samples: usize) -> Vec<Self::State> {
        let mut rng = thread_rng();
        let samples: Vec<Point3<f32>> = (&mut rng)
            .sample_iter(Standard)
            .take(num_samples)
            .map(|(x, y, z): (f32, f32, f32)| {
                Point3::new(x * self.size.x, y * self.size.y, z * self.size.z)
            })
            .collect();
        samples
    }
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
