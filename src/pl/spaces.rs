use super::{State, StateSpace};
use nalgebra::{Point2, Point3, Vector2, Vector3};
use rand::{distributions::Standard, thread_rng, Rng};

impl State for Point2<f32> {
    fn dist(&self, other: &Self) -> f32 {
        (self - other).norm()
    }

    fn project_to_3d(&self) -> [f32; 3] {
        [self.x, self.y, 0.0]
    }
}

#[derive(Debug)]
pub struct RectangleSpace {
    pub size: Vector2<f32>,
}

impl StateSpace for RectangleSpace {
    type State = Point2<f32>;

    fn sample_batch(&self, num_samples: usize) -> Vec<Self::State> {
        let mut rng = thread_rng();
        let samples: Vec<Point2<f32>> = (&mut rng)
            .sample_iter(Standard)
            .take(num_samples)
            .map(|(x, y): (f32, f32)| Point2::new(x * self.size.x, y * self.size.y))
            .collect();
        samples
    }
}

#[derive(Debug)]
pub struct CircleSpace {
    pub radius: f32,
}

impl StateSpace for CircleSpace {
    type State = Point2<f32>;

    fn sample_batch(&self, num_samples: usize) -> Vec<Self::State> {
        let mut rng = thread_rng();
        let samples: Vec<Point2<f32>> = (&mut rng)
            .sample_iter(Standard)
            .take(num_samples)
            .map(|(r, theta): (f32, f32)| {
                let r = r.sqrt();
                Point2::new(
                    r * self.radius * (theta * 2.0 * std::f32::consts::PI).cos(),
                    r * self.radius * (theta * 2.0 * std::f32::consts::PI).sin(),
                )
            })
            .collect();
        samples
    }
}

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

#[derive(Debug)]
pub struct SphereSpace {
    pub radius: f32,
}

impl StateSpace for SphereSpace {
    type State = Point3<f32>;

    fn sample_batch(&self, num_samples: usize) -> Vec<Self::State> {
        let mut rng = thread_rng();
        let samples: Vec<Point3<f32>> = (&mut rng)
            .sample_iter(Standard)
            .take(num_samples)
            .map(|(r, phi, theta): (f32, f32, f32)| {
                let r = r.sqrt();
                Point3::new(
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
