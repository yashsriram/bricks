use bevy::render::mesh::Mesh;
use std::fmt::Debug;

pub mod graph;

pub trait State: Debug {
    fn dist(&self, other: &Self) -> f32;

    fn project_to_3d(&self) -> [f32; 3];
}

pub trait StateSpace: Debug + Into<Mesh> {
    type State: State;

    fn sample_batch(&self, num_samples: usize) -> Vec<Self::State>;
}

pub mod spaces {
    use super::{State, StateSpace};
    use bevy::prelude::*;
    use bevy::render::mesh::{Indices, Mesh};
    use bevy::render::pipeline::PrimitiveTopology;
    use rand::{distributions::Standard, thread_rng, Rng};

    impl State for Vec2 {
        fn dist(&self, other: &Self) -> f32 {
            (*self - *other).length()
        }

        fn project_to_3d(&self) -> [f32; 3] {
            [self.x, self.y, 0.0]
        }
    }

    #[derive(Debug)]
    pub struct RectangleSpace {
        pub size: Vec2,
    }

    impl StateSpace for RectangleSpace {
        type State = Vec2;

        fn sample_batch(&self, num_samples: usize) -> Vec<Self::State> {
            let mut rng = thread_rng();
            let samples: Vec<Vec2> = (&mut rng)
                .sample_iter(Standard)
                .take(num_samples)
                .map(|(x, y): (f32, f32)| Vec2::new(x * self.size.x, y * self.size.y))
                .collect();
            samples
        }
    }

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
            let colors = vec![[0.0, 1.0, 1.0, 0.1]; 4];
            mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);
            mesh
        }
    }

    #[derive(Debug)]
    pub struct CircleSpace {
        pub radius: f32,
    }

    impl StateSpace for CircleSpace {
        type State = Vec2;

        fn sample_batch(&self, num_samples: usize) -> Vec<Self::State> {
            let mut rng = thread_rng();
            let samples: Vec<Vec2> = (&mut rng)
                .sample_iter(Standard)
                .take(num_samples)
                .map(|(r, theta): (f32, f32)| {
                    let r = r.sqrt();
                    Vec2::new(
                        r * self.radius * (theta * 2.0 * std::f32::consts::PI).cos(),
                        r * self.radius * (theta * 2.0 * std::f32::consts::PI).sin(),
                    )
                })
                .collect();
            samples
        }
    }

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
            let colors = vec![[0.0, 1.0, 1.0, 0.1]; num_partitions + 1];
            mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);
            mesh
        }
    }

    impl State for Vec3 {
        fn dist(&self, other: &Self) -> f32 {
            (*self - *other).length()
        }

        fn project_to_3d(&self) -> [f32; 3] {
            [self.x, self.y, self.z]
        }
    }

    #[derive(Debug)]
    pub struct CuboidSpace {
        pub size: Vec3,
    }

    impl StateSpace for CuboidSpace {
        type State = Vec3;

        fn sample_batch(&self, num_samples: usize) -> Vec<Self::State> {
            let mut rng = thread_rng();
            let samples: Vec<Vec3> = (&mut rng)
                .sample_iter(Standard)
                .take(num_samples)
                .map(|(x, y, z): (f32, f32, f32)| {
                    Vec3::new(x * self.size.x, y * self.size.y, z * self.size.z)
                })
                .collect();
            samples
        }
    }

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
            let vertex_colors = vec![[0.0, 1.0, 1.0, 0.1]; 8];
            mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);
            mesh
        }
    }

    #[derive(Debug)]
    pub struct SphereSpace {
        pub radius: f32,
    }

    impl StateSpace for SphereSpace {
        type State = Vec3;

        fn sample_batch(&self, num_samples: usize) -> Vec<Self::State> {
            let mut rng = thread_rng();
            let samples: Vec<Vec3> = (&mut rng)
                .sample_iter(Standard)
                .take(num_samples)
                .map(|(r, phi, theta): (f32, f32, f32)| {
                    let r = r.sqrt();
                    Vec3::new(
                        r * self.radius
                            * (theta * 2.0 * std::f32::consts::PI).cos()
                            * (phi * 2.0 * std::f32::consts::PI).cos(),
                        r * self.radius
                            * (theta * 2.0 * std::f32::consts::PI).cos()
                            * (phi * 2.0 * std::f32::consts::PI).sin(),
                        r * self.radius * (theta * 2.0 * std::f32::consts::PI).sin(),
                    )
                })
                .collect();
            samples
        }
    }

    impl From<SphereSpace> for Mesh {
        fn from(s: SphereSpace) -> Mesh {
            let mut mesh: Mesh = shape::Icosphere {
                radius: s.radius,
                subdivisions: 10,
            }
            .into();
            mesh.set_attribute(
                Mesh::ATTRIBUTE_COLOR,
                vec![[0.0, 1.0, 1.0, 0.1]; mesh.count_vertices()],
            );
            mesh
        }
    }
}
