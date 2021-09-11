use bevy::render::mesh::Mesh;
use std::fmt::Debug;

pub mod graph;
pub mod spaces;

pub trait State: Debug {
    fn dist(&self, other: &Self) -> f32;

    fn project_to_3d(&self) -> [f32; 3];
}

pub trait StateSpace: Debug + Into<Mesh> {
    type State: State;

    fn sample_batch(&self, num_samples: usize) -> Vec<Self::State>;
}
